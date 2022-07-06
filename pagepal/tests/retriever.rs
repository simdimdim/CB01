use pagepal_ui::*;
use std::path::PathBuf;
use tokio;

#[tokio::test]
async fn fetch() {
    let p = "https://readmanganato.com/manga-fz982434/chapter-16".into();
    let r = Retriever::default();
    let page = r.get(p).await;
    let _next = r.next(&page).await;
    // let index = r.index(&next.unwrap()).await;
    // let _ = r.links(&index).await;
}

#[tokio::test]
async fn dl_test() {
    let r = Retriever::default();
    let mut p:Page = "https://s18.mkklcdnv6tempv4.com/mangakakalot/k1/kanojo_okarishimasu/chapter_7/2.jpg".into();
    p = r.get(p).await;
    let mut c = Content::Image {
        pb:  PathBuf::from("library/picture.jpg"),
        src: Some(p.url.clone()),
    };
    // tokio::fs::write("library/pivture2.jpg", &*p.image(&r.client).await)
    //     .await
    //     .unwrap();
    // c.save(&p.image(&r.client).await).await
}
