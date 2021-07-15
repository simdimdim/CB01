use crate::{Book, Content, Label};
use chrono::Duration;
use futures::future::{join_all, OptionFuture};
use reqwest::{Client, Url};
use sites::Include;
use std::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
        HashMap,
    },
    convert::TryInto,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use url::Host;

pub mod delay;
pub mod finder;
pub mod page;
pub mod sites;

pub use delay::*;
pub use finder::*;
pub use page::*;

#[derive(Debug, Clone)]
pub struct Retriever {
    pub client: Client,
    pub jar:    Arc<reqwest::cookie::Jar>,
    pub delays: Arc<Mutex<BTreeMap<Host, Delay>>>,
    pub sites:  Arc<HashMap<Host, Box<dyn Finder>>>,
    pub finder: Arc<Box<dyn Finder>>, // default
}

impl Retriever {
    pub async fn get(&self, mut p: Page) -> Page {
        if p.req.is_none() {
            let h = self.finder(&p).headers();
            let req = self.client.get(p.url.clone()).headers(h).build().unwrap();
            p.prep(req);
        }
        self.access(&p).await;
        p.fetch(&self.client).await
    }

    pub fn num(&self, page: &Page) -> (u16, u16, String) {
        page.num(self.finder(page))
    }

    pub async fn title(&self, page: &Page) -> Label {
        page.title(self.finder(page)).await
    }

    pub async fn index(&self, page: &Page) -> Page {
        let res: OptionFuture<_> = page
            .index(self.finder(page))
            .await
            .map(|a| self.get(a))
            .into();
        res.await.expect("Couldn't resolve index")
    }

    pub async fn links(&self, page: &Page) -> Vec<Page> {
        let mut res = vec![];
        for p in page.links(self.finder(page)).await {
            res.push(self.get(p).await);
        }
        res
    }

    pub async fn next(&self, page: &Page) -> Option<Page> {
        let res: OptionFuture<_> = page
            .next(self.finder(page))
            .await
            .map(|a| self.get(a))
            .into();
        res.await
    }

    pub async fn text(&self, page: &Page) -> Vec<String> {
        page.text(self.finder(page)).await
    }

    pub async fn images(&self, page: &Page) -> Vec<Page> {
        // use tokio::time::sleep;
        // tokio::time::sleep(std::time::Duration::from_secs_f32(0.250)). await;
        // let mut res = vec![];
        join_all(
            page.images(self.finder(page))
                .await
                .into_iter()
                .map(|p| async move { self.get(p).await }),
        )
        .await
    }

    fn finder(&self, p: &Page) -> &Box<dyn Finder> {
        self.sites.get(&p.domain()).unwrap_or(&*self.finder)
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

    pub async fn new_book(&self, url: Url) -> (Label, Book) {
        let init = self.get(url.into()).await;
        let title = self.title(&init).await;
        let mut index = self.index(&init).await;
        index.empty();
        let mut bk = Book::new(Some(index));
        let mut images = self.images(&init).await;
        //        images = images.into_iter().take(1).collect::<Vec<_>>();
        images.iter_mut().for_each(|p| p.empty());
        bk.chap_add(None, images.len())
            .set_src(Some(init.url.clone()));
        bk.chapters[0].offset = 1;
        bk.chapters[0].len = images.len() as crate::Id;
        // for batch dls take a look at:
        // https://gist.github.com/mtkennerly/b513e7fe89c735e5a5df672c503404d7#file-main-rs-L42
        let name = || title.clone();
        let cnt =
            join_all(images.into_iter().enumerate().map(|(n, p)| async move {
                let num = self.num(&p);
                let path = PathBuf::from("library")
                    .join(name().0)
                    .join(format!("{:04}", num.1));
                std::fs::create_dir_all(&path).unwrap();
                let mut content = Content::Image {
                    pb:  path.join(format!("{:04}", n)),
                    src: Some(p.url.clone()),
                };
                content.save(&p.image(&self.client).await).await;
                content
            }))
            .await;
        bk.cont_add(cnt, None);
        (title, bk)
    }
}

impl Default for Retriever {
    fn default() -> Self {
        let mut hm: HashMap<Host, Box<dyn Finder>> = HashMap::new();
        Include::custom(&mut hm);
        static AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0";
        let ja = Arc::new(reqwest::cookie::Jar::default());
        let cl = Client::builder()
            .user_agent(AGENT)
		.connection_verbose(true)
            .cookie_provider(ja.clone())
            .cookie_store(true)
            .http2_adaptive_window(true)
            .http2_max_frame_size(Some(u16::MAX as u32 * u8::MAX as u32))
            .build()
            .unwrap();
        Self {
            client: cl,
            jar:    ja,
            delays: Default::default(),
            sites:  Arc::new(hm),
            finder: Arc::new(Box::new(DefaultFinder)),
        }
    }
}
