use reqwest::{Client, Url};
use select::document::Document;

pub mod finders;
pub mod page;

pub use finders::*;
pub use page::*;

#[derive(Debug, Clone, Default)]
pub struct Retriever {
    client: Client,
}
impl Retriever {
    pub async fn fetch(&self, url: Url) -> Document {
        let html = self
            .client
            .get(url.to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        Document::from(html.as_ref())
    }

    pub async fn text<T: TextFinder>(
        &self, _page: Page, _finder: Option<T>,
    ) -> Vec<String> {
        vec![]
    }
}
