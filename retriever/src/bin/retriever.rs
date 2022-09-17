#![feature(iter_advance_by)]
use clap::Parser;
use futures::future::join_all;
use log::{debug, info, trace};
use reqwest::Url;
use retriever::{
    hounds::presets::{fetch, fetch_one, rr_text, Extractor},
    page::ContentType,
};
use static_init::dynamic;
use std::{fmt::Debug, io, path::PathBuf, time::Duration};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Opt {
    #[clap(short, long, value_parser)]
    /// String contained in the next page button
    next: Option<String>,
    #[clap(short, long, value_parser)]
    /// String to split the title by to get novel/manga title
    split: Option<String>,
    #[clap(short, long, value_parser)]
    /// Output directory
    out_dir: Option<PathBuf>,
    #[clap(
        short,
        long = "manga",
        required(true),
        group = "kind",
        conflicts_with = "novel"
    )]
    /// Grab images
    image: bool,
    #[clap(short = 't', long, required(true), group = "kind")]
    /// Grab text
    novel: bool,
    #[clap(min_values(1))]
    /// url/s to manga or novels
    urls: Vec<Url>,
    #[clap(short, long, value_parser, default_value = "400")]
    /// How long to sleep [in milliseconds] between requesting pages
    delay: u64,
    #[clap(short, long, group = "text")]
    /// Set dedicated text extractor for RR
    royalroad: bool,
}

#[dynamic(lazy)]
static mut NEXT: &'static str = "A";
#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let args = Opt::parse();
    let mut extractor = Extractor::default();
    if args.royalroad {
        extractor.set_text(Some(rr_text));
    }
    if let Some(dir) = &args.out_dir {
        std::fs::create_dir_all(dir).expect("Failed to create path to output directory.");
    }
    if let Some(next) = args.next {
        info!("Set string to look for next chapter link to: \"{}\"", &next);
        let mut s = NEXT.write();
        *s = Box::leak(next.into_boxed_str());
    }
    if let Some(_split) = args.split {}
    info!("delay: {}", &args.delay);
    info!("Looking for {}", if args.image { "images" } else { "text" });
    let mut content = vec![];
    let mut page = args.urls.into_iter().map(Into::into).into_iter().next();
    // fetch(pages.as_mut(), &extractor, args.text).await;
    while let Some(mut p) = page {
        trace!("Delayed: {}", &args.delay);
        fetch_one(&mut p, &extractor, args.image).await;
        if let Some(cnt) = p.content.data.take() {
            let path = args.out_dir.clone().unwrap();
            debug!("path base: {:?}", path);
            match cnt {
                ContentType::Text(_, _) => {
                    let final_path = path.clone().join(p.chapter());
                    debug!("path with page name: {:?}", final_path);
                    p.content.data = Some(cnt);
                    p.content.save(final_path).await.unwrap();
                }
                images @ ContentType::Images(_, _) => {
                    if let Some(mut res) = images.to_pages() {
                        fetch(&mut res, &extractor, args.image).await;
                        join_all(res.iter().map(|c| async {
                            let final_path = path.clone().join(p.chapter());
                            debug!("path with chapter: {:?}", final_path);
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
        tokio::time::sleep(Duration::from_millis(args.delay)).await;
    }
    debug!("Gathered {:?} pages", content);
    Ok(())
}
