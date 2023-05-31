use clap::Parser;
use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};
use futures::future::join_all;
use log::{debug, info, trace};
use reqwest::Url;
use retriever::{
    extractor::Extractor,
    page::{Page, SepStr},
    presets::{realm_images, realm_index, realm_next},
    retriever::Retriever,
};
use std::{ffi::OsString, fmt::Debug, fs, fs::OpenOptions, io, path::PathBuf, time::Duration};

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
    #[clap(short, long, value_parser, display_order(3))]
    /// Output directory
    output_dir: Option<PathBuf>,
    #[clap(short, long, value_parser, default_value = "400", display_order(4))]
    /// Delay interval between page requests (ms)
    delay: u64,
    #[clap(short, long, value_parser, display_order(5))]
    /// String contained in the next page button
    next: Option<String>,
    #[clap(num_args(1))]
    /// url to manga or novels
    url: Url,
    #[clap(long, group = "text", display_order(6))]
    /// Use RealmScans specific extractors
    realm: bool,
    #[clap(short, long, value_parser, conflicts_with = "image", display_order(7))]
    /// Generate epub
    epub: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "warn,retriever=debug");
    env_logger::init();

    let args = Opt::parse();
    let mut extractor = Extractor::default();
    let ret = Retriever::default();
    if args.realm {
        extractor.set_next(Some(realm_next));
        extractor.set_index(Some(realm_index));
        extractor.set_images(Some(realm_images));
    }
    if let Some(dir) = &args.output_dir {
        std::fs::create_dir_all(dir).expect("Failed to create path to output directory.");
    }
    let sep = if let Some(next) = args.next {
        info!("Next chapter button string: '{}'", &next);
        let s: &'static str = Box::leak(next.into_boxed_str());
        SepStr::from(s)
    } else {
        Default::default()
    };
    info!("Delay: {}", &args.delay);
    info!("Looking for {}", if args.image { "images" } else { "text" });
    let mut page: Page = args.url.try_into().unwrap();
    page.set_next(sep);
    let mut all_imgs = vec![];
    if args.novel {
        debug!("current at : {:?}", page.url);
        let mut next = Some(page);
        while let Ok(u) = ret
            .fetch_next(&mut next.unwrap_or_default(), args.image)
            .await
        {
            tokio::time::sleep(Duration::from_millis(args.delay)).await;
            debug!("current at : {:?}", u.url);
            ret.fetch_content(u, args.image).await;
            next = u.next();
            all_imgs.push(u.clone());
        }
    } else if let Ok(index) = ret.fetch_index(&mut page, args.image).await {
        if let Ok(links) = ret.fetch_links(index, args.image).await {
            debug!("Fetched {:?} chapters", links.content.data);
            tokio::time::sleep(Duration::from_millis(args.delay)).await;
            if args.image {
                if let Some(chapters) = ret.fetch_content(links, args.image).await {
                    for mut chapter in chapters {
                        if let Some(images) = ret.fetch_content(&mut chapter, args.image).await {
                            debug!("Gathered {} images", images.len());
                            all_imgs.extend(images);
                        }
                    }
                }
            } else if let Some(chapters) = ret.fetch_content(links, args.image).await {
                trace!("Gathered {} chapters", chapters.len());
                all_imgs.extend(chapters);
            }
        }
    }
    let save_to = args.output_dir.unwrap_or_else(|| PathBuf::from("./"));
    join_all(all_imgs.chunks_mut(5).map(|a| async {
        for p in a {
            tokio::time::sleep(Duration::from_millis(args.delay + 100)).await;
            ret.fetch(p, &extractor, args.image).await;
            p.save(save_to.as_path()).await.unwrap();
        }
    }))
    .await;
    info!("Total {} pages", all_imgs.len());

    if args.epub {
        gen_epub_for(save_to);
    }
    Ok(())
}

pub fn gen_epub_for(pb: PathBuf) {
    if let Ok(dir) = fs::read_dir(&pb) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(pb.join("book.epub"))
            .unwrap();
        let mut book = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
        book.metadata("author", " ")
            .unwrap()
            .metadata("title", " ")
            .unwrap()
            .metadata("lang", "en-GB")
            .unwrap()
            .inline_toc();
        let mut img_tags = vec![];
        dir.flatten().for_each(|f| {
            if f.path().extension().unwrap() != "epub" {
                let image = OpenOptions::new().read(true).open(f.path()).unwrap();
                let mut img_path = OsString::from(""); //
                img_path.push(f.path().file_name().unwrap());
                debug!("{:?}", &img_path);
                book.add_resource(
                    &img_path,
                    image,
                    format!("image/{}", f.path().extension().unwrap().to_str().unwrap()),
                )
                .unwrap();
                img_tags.push(format!(
                    r##"<img src="{0}" alt="{0}" />"##,
                    img_path.to_str().unwrap()
                ));
            }
        });
        book.add_content(EpubContent::new(
            "Content.xhtml",
            format!(
                r##"
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
  <title>Book</title>
  <head>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <div>
      {}
    </div>
  </body>
</html>"##,
                {
                    img_tags.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    img_tags.join("\n")
                }
            )
            .as_bytes(),
        ))
        .unwrap();
        book.generate(&mut file).unwrap();
    }
}
