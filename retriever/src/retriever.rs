use crate::{
    extractor::{Extractor, Manifest},
    page::{ContentType, Page},
};
use core::fmt::Debug;
use dashmap::DashMap;
use futures::future::join_all;
#[allow(unused_imports)]
use log::{debug, info, trace};
use reqwest::{redirect::Policy, Client};
use std::time::Duration;
use url::Host;

pub type TitleType = String;
pub type NextType = Option<Page>;
pub type TextType = Vec<String>;
pub type ImagesType = Vec<Page>;
pub type LinksType = Vec<Page>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Retriever {
    extractors: DashMap<Host, usize>,
    targets: DashMap<Host, Page>,
    manifests: Vec<Manifest>,
    extr: Vec<Extractor>,
    default_extractor: usize,
    client: Client,
}

#[allow(unused_variables)]
impl Retriever {
    pub fn new() -> Self {
        #[allow(clippy::needless_update)]
        Self {
            extractors: DashMap::new(),
            targets: DashMap::new(),
            manifests: vec![],
            client: Client::default(),
            ..Default::default()
        }
    }

    pub async fn fetch_all(
        &self, pages: &mut [Page], extractor: &Extractor, visual: bool, delay: u64,
    ) {
        join_all(pages.chunks_mut(10).map(|p| async {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            for i in p {
                self.fetch(i, extractor, visual).await;
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }))
        .await;
    }

    pub async fn fetch(&self, page: &mut Page, extractor: &Extractor, visual: bool) {
        if let Some(time) = page.last {
            if time.minute() < 1 {
                info!("Visited recently");
                return;
            }
        }
        page.visit(self.client.clone(), extractor, visual).await;
    }

    pub async fn fetch_index<'a: 'b, 'b>(
        &self, page: &'a mut Page, kind: bool,
    ) -> Result<&'b mut Page, &'a mut Page> {
        self.check_page(page, kind).await;
        if let Some(mut p) = page
            .content
            .index()
            .as_ref()
            .and_then(|p| Page::try_from(p).ok())
        {
            p.next_by = page.next_by;
            *page = p;
            return Ok(page);
        }
        Err(page)
    }

    pub async fn fetch_next<'a: 'b, 'b>(
        &self, page: &'a mut Page, kind: bool,
    ) -> Result<&'b mut Page, &'a mut Page> {
        self.check_page(page, kind).await;
        if let Some(mut next) = page.next() {
            next.next_by = page.next_by;
            *page = next;
            return Ok(page);
        }
        Err(page)
    }

    pub async fn fetch_links<'a: 'b, 'b>(
        &self, page: &'a mut Page, kind: bool,
    ) -> Result<&'b mut Page, &'a mut Page> {
        self.check_page(page, kind).await;
        let cnt = page
            .content
            .links()
            .as_ref()
            .map(|v| ContentType::Chapters(v.to_owned(), Some(page.origin()))); // FIXME: don't clone
        if cnt.is_some() {
            debug!("{:?}", cnt);
            page.content.data = cnt;
            return Ok(page);
        }
        Err(page)
    }

    pub async fn fetch_content(&self, page: &mut Page, kind: bool) -> Option<Vec<Page>> {
        self.check_page(page, kind).await;
        let out = page.content.data.as_ref().and_then(|p| {
            debug!("{:?}", &p.to_pages());
            p.to_pages()
        });

        out
    }

    pub async fn check_page(&self, page: &mut Page, kind: bool) {
        if page.last.is_none() {
            self.fetch(page, &self.extr[self.default_extractor], kind)
                .await;
        }
    }

    //TODO: extract all data from a Page at the same time

    pub fn add_extractor(&mut self, from: Option<usize>) -> bool {
        if let Some(r) = self.manifests.get(from.unwrap_or(0)) {
            self.manifests.insert(self.manifests.len(), r.clone());
            return true;
        }
        false
    }

    /// false if text, true if images
    pub async fn guess_type(&self, _src: &Page) -> bool { false }
}

impl Default for Retriever {
    fn default() -> Self {
        let extractors = DashMap::new();
        let targets = DashMap::new();
        let m: Manifest = {
            let mut m = Manifest::default();
            m.rename("Default");
            m
        };
        let manifests: Vec<Manifest> = vec![m];
        let client = Client::builder()
            .connection_verbose(true)
            .cookie_store(true)
            .http2_adaptive_window(true)
            .redirect(Policy::default())
            .build()
            .unwrap();
        Self {
            extractors,
            targets,
            manifests,
            default_extractor: 0,
            client,
            extr: vec![Default::default()],
        }
    }
}
