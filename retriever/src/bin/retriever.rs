use clap::Parser;
use futures::future::join_all;
use log::{debug, info, trace};
use reqwest::Url;
use retriever::{
    extractor::Extractor,
    page::{fetch, fetch_one, ContentType, Page, SepStr},
    presets::{realm_images, realm_next},
};
use std::{fmt::Debug, io, path::PathBuf, time::Duration};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Opt {
    #[clap(
        short,
        long = "manga",
        required_unless_present = "novel",
        group = "kind",
        conflicts_with = "novel",
        display_order(1)
    )]
    /// Look for images
    image: bool,
    #[clap(
        short = 't',
        long,
        required_unless_present = "image",
        group = "kind",
        display_order(2)
    )]
    /// Look for text
    novel: bool,
    #[clap(long, group = "text", display_order(6))]
    /// Use RealmScans specific extractors
    realm: bool,
    #[clap(min_values(1), max_values(1))]
    /// url to manga or novels
    url: Url,
    #[clap(short, long, value_parser, display_order(3), next_display_order = "3")]
    /// Output directory
    output_dir: Option<PathBuf>,
    #[clap(short, long, value_parser, display_order(5))]
    /// String contained in the next page button
    next: Option<String>,
    #[clap(short, long, value_parser, default_value = "400", display_order(4))]
    /// Sleep interval between page requests (ms)
    sleep: u64,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "warn,retriever=debug");
    env_logger::init();

    let args = Opt::parse();
    let mut extractor = Extractor::default();
    if args.realm {
        extractor.set_next(Some(realm_next));
        extractor.set_images(Some(realm_images));
        debug!("jsbs is enabled");
    }
    if let Some(dir) = &args.output_dir {
        std::fs::create_dir_all(dir).expect("Failed to create path to output directory.");
    }
    let sep = if let Some(next) = args.next {
        info!("Set string to look for next chapter link to: \"{}\"", &next);
        let s: &'static str = Box::leak(next.into_boxed_str());
        SepStr::from(s)
    } else {
        Default::default()
    };
    info!("delay: {}", &args.sleep);
    info!("Looking for {}", if args.image { "images" } else { "text" });
    let mut content = vec![];
    let mut page: Option<Page> = args.url.try_into().ok();
    while let Some(mut p) = page.as_mut() {
        trace!("Delayed: {}", &args.sleep);
        p.set_next(sep);
        fetch_one(p, &extractor, args.image).await;
        if let Some(cnt) = p.content.data.take() {
            let path = args.output_dir.clone().unwrap();
            match cnt {
                ContentType::Text(_, _) => {
                    let final_path = path.clone().join(p.chapter());
                    p.content.data = Some(cnt);
                    p.content.save(final_path).await.unwrap();
                }
                images @ ContentType::Images(_, _) => {
                    if let Some(mut res) = images.to_pages() {
                        fetch(&mut res, &extractor, args.image).await;
                        join_all(res.iter().map(|c| async {
                            let final_path = path.clone().join(p.chapter());

                            c.content.save(final_path).await
                        }))
                        .await;
                        content.push(res);
                    }
                }
                _ => {}
            };
        };
        page = p.next();
        debug!("Next page: {:?}", &page);
        tokio::time::sleep(Duration::from_millis(args.sleep)).await;
    }
    debug!("Gathered {:?} pages", content.len());
    Ok(())
}
