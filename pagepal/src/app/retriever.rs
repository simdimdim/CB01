use crate::{Book, Chapter, Content, Id, Label};
use chrono::Duration;
use futures::future::join_all;
use reqwest::{cookie::Jar, Client, Url};
use sites::Include;
use std::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock};
use url::Host;

pub mod delay;
pub mod finder;
pub mod page;
pub mod sites;

pub use delay::*;
pub use finder::*;
pub use page::*;

type Find = Box<dyn Finder>;

#[derive(Debug, Clone)]
pub struct Retriever {
    pub client: Client,
    pub jar:    Arc<Jar>,
    pub delays: Arc<Mutex<BTreeMap<Host, Delay>>>,
    pub find:   Arc<RwLock<Vec<Find>>>,
    pub hosts:  Arc<RwLock<HashMap<Host, usize>>>,
}

impl Default for Retriever {
    fn default() -> Self {
        static AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0";
        let ja = Arc::new(Jar::default());
        let cl = Client::builder()
            .user_agent(AGENT)
            .connection_verbose(true)
            .cookie_provider(ja.clone())
            .cookie_store(true)
            .http2_adaptive_window(true)
            .build()
            .unwrap();
        let mut find: Vec<Box<dyn Finder>> = vec![Box::new(DefaultFinder)];
        let mut hosts = HashMap::new();
        Include::custom(&mut find, &mut hosts);
        Self {
            client: cl,
            jar:    ja,
            delays: Default::default(),
            find:   Arc::new(RwLock::new(find)),
            hosts:  Arc::new(RwLock::new(hosts)),
        }
    }
}
/// Code duplication preventive measure
macro_rules! find {
    ($this:ident,$a:expr) => {{
        let idx = $this
            .hosts
            .read()
            .await
            .get(&$a.domain())
            .copied()
            .unwrap_or_default();
        &$this.find.read().await[idx]
    }};
}
impl Retriever {
    pub async fn get(&self, mut p: Page) -> Page {
        if p.req.is_none() {
            let h = find!(self, p).headers();
            let req = self.client.get(p.url.clone()).headers(h).build().unwrap();
            p.prep(req);
        }
        self.access(&p).await;
        p.fetch(&self.client).await
    }

    pub async fn num(&self, page: &Page) -> (u16, u16, String) {
        page.num(find!(self, page))
    }

    pub async fn title(&self, page: &Page) -> Label {
        page.title(find!(self, page)).await
    }

    pub async fn index(&self, page: &Page) -> Page {
        let finder = find!(self, page);
        if let Some(a) = page.index(finder).await {
            self.get(a).await
        } else {
            page.to_owned()
        }
    }

    pub async fn links(&self, page: &Page) -> Vec<Page> {
        let mut res = vec![];
        for p in page.links(find!(self, page)).await {
            res.push(self.get(p).await);
        }
        res
    }

    pub async fn next(&self, page: &Page) -> Option<Page> {
        let finder = find!(self, page);
        if let Some(a) = page.next(finder).await {
            Some(self.get(a).await)
        } else {
            None
        }
    }

    pub async fn text(&self, page: &Page) -> Vec<String> {
        page.text(find!(self, page)).await
    }

    pub async fn images(&self, page: &Page) -> Vec<Page> {
        // use tokio::time::sleep;
        // tokio::time::sleep(std::time::Duration::from_secs_f32(0.250)). await;
        let finder = find!(self, page);
        let headers = finder.headers();
        let images = page.images(finder);
        for p in images.iter() {
            self.add_related(page, p).await;
        }
        images.into_iter().fold(vec![], |mut res, mut p| {
            p.prep(
                self.client
                    .get(p.url.clone())
                    .headers(headers.clone())
                    .build()
                    .unwrap(),
            );
            res.push(p);
            res
        })
    }

    pub async fn new_book(&self, url: Url) -> (Label, Box<Book>) {
        let init = self.get(url.into()).await;
        let title = self.title(&init).await;
        let mut index = self.index(&init).await;
        index.empty();
        let mut bk = Book::new(Some(index));
        // TODO: determine if init is chapter or index and act appropriately
        let images = self.images(&init).await;
        bk.chap_add_from_parts(None, images.len())
            .set_src(Some(init.url.clone()));
        (title, Box::new(bk))
    }

    pub async fn new_chapter(&self, page: &Page) -> Chapter {
        Chapter {
            name: Some(self.title(page).await),
            src: Some(page.url.clone()),
            ..Default::default()
        }
    }

    pub async fn chap_dl(&self, mut ch: Chapter) -> Vec<Content> {
        if let Some(url) = ch.src {
            let page = self.get(url.into()).await;
            let mut images = self.images(&page).await;
            ch.len = images.len() as Id;
            let name = &ch.name.unwrap().deref().clone();
            let res = join_all(images.iter_mut().enumerate().map(
                |(n, p)| async move {
                    let num = self.num(p).await;
                    let path = PathBuf::from("library")
                        .join(name)
                        .join(format!("{:04}", num.1));
                    std::fs::create_dir_all(&path).unwrap();
                    let mut content = Content::Image {
                        pb:  path.join(format!("{:04}", n)),
                        src: Some(p.url.clone()),
                    };
                    content.save(&p.image(&self.client).await).await;
                    p.empty();
                    content
                },
            ))
            .await;
            ch.full = true;
            return res;
        }
        vec![]
    }

    pub async fn add_host(&mut self, host: Host, idx: usize) {
        self.hosts.write().await.insert(host, idx);
    }

    pub async fn add_related(&self, from: &Page, rel: &Page) {
        let a = self.hosts.read().await.get(&from.domain()).copied();
        if let Some(e) = a {
            let mut rw = self.hosts.write().await;
            rw.entry(rel.domain()).or_insert(e);
        };
    }

    pub async fn add_related_batch(&self, from: &Page, rel: Vec<&Page>) {
        if self.hosts.read().await.contains_key(&from.domain()) {
            for u in rel.into_iter() {
                self.add_related(from, u).await;
            }
        }
    }

    /// Keeps track of domains being accessed and adds delay between accessed
    async fn access(&self, p: &Page) {
        match self.delays.lock().await.entry(p.domain()) {
            Occupied(mut e) => {
                e.get_mut().delay_if(Duration::milliseconds(100)).await;
            }
            Vacant(e) => {
                e.insert(Default::default());
            }
        }
        // TODO: Maybe add a trim function for the map that runs occasionally
    }
}
