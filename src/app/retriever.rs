use crate::{Book, Label};
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
        let mut res = vec![];
        for p in page.images(self.finder(page)).await {
            // sleep(std::time::Duration::from_secs_f32(0.125)).await;
            res.push(self.get(p).await);
        }
        res
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
        let index = self.index(&init).await;
        let mut bk = Book::new(Some(index));
        let images = init.images(self.finder(&init).clone()).await;
        bk.chap_add(None, images.len());
        // for batch dls take a look at:
        // https://gist.github.com/mtkennerly/b513e7fe89c735e5a5df672c503404d7#file-main-rs-L42
        let cnt = join_all(
            images
                .into_iter()
                .map(|p| async move { p.novel(self.finder(&p)).await }),
        )
        .await;
        bk.cont_add(cnt, None);
        (title, bk)
    }
}

impl Default for Retriever {
    fn default() -> Self {
        let mut hm: HashMap<Host, Box<dyn Finder>> = HashMap::new();
        Include::custom(&mut hm);
        Self {
            client: Default::default(),
            delays: Default::default(),
            sites:  Arc::new(hm),
            finder: Arc::new(Box::new(DefaultFinder)),
        }
    }
}
