use crate::{
    page::{ContentType, Page},
    Images,
    Index,
    Links,
    Next,
    Text,
    Title,
};
#[allow(unused_imports)]
use log::debug;
use select::predicate::{And, Any, Attr, Child, Descendant, Name, Or, Text as Txt};

pub fn default_title(page: &Page) -> Title {
    page.doc().map(|d| {
        let title = d
            .select(Name("title"))
            .into_selection()
            .first()
            .unwrap()
            .text();
        if title.contains(page.split().as_str()) {
            title
                .split(page.split().as_str())
                .filter(|&a| !a.is_empty())
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string()
        } else {
            title
        }
    })
}
pub fn default_next(page: &Page) -> Next {
    page.doc().and_then(|d| {
        d.select(Child(Name("a"), Txt))
            .filter(|a| a.text().contains(page.get_next()))
            .map(|a| a.parent().unwrap().attr("href").unwrap().to_string())
            .next()
    })
}
pub fn default_index(page: &Page) -> Index {
    let _ = page.doc(); //.map(|d| d);
    // Some(String::from("https://manganato.com/manga-ig985463"))
    let mut index = page.url.as_str().split('/');
    index.advance_back_by(3).unwrap();
    index.collect::<Vec<_>>().join("/").parse().ok()
}
pub fn default_links(page: &Page) -> Links {
    // let c =
    // debug!("a: {:?}", &page.url.as_str());
    // debug!("a: {:?}", &page.html);
    page.doc().map(|d| {
        d.select(Descendant(
            Name("div"),
            Or(Name("p"), Or(Name("table"), Name("ul"))),
        ))
        .map(|a| a.select(Name("a")).into_selection())
        .max_by(|a, b| {
            // debug!("a: {:?} b: {:?}", &a.len(), &b.len());
            a.len().cmp(&b.len())
        })
        .unwrap()
        .iter()
        .filter_map(|a| a.attr("href"))
        .map(|a| a.to_string())
        .map(Into::into)
        .collect()
    })
    // ;
    // debug!("{:?}", &c);
    // c
}
pub fn default_text(page: &Page) -> Text {
    page.doc().map(|d| {
        // debug!(
        //     "{:?}",
        //     d.select(Or(Descendant(Any, Name("p")), Descendant(Any, Name("br"))))
        //         .map(|a| a.parent().unwrap().children().into_selection())
        //         .max_by(|a, b| {
        //             debug!("len a: {:?}, len b: {:?}", a.len(), b.len());
        //             a.len().cmp(&b.len())
        //         })
        //         .unwrap()
        //         .select(Txt)
        //         .iter()
        //         .map(|a| a.text())
        //         .collect::<Vec<_>>()
        // );
        ContentType::Text(
            d.select(Or(Descendant(Any, Name("p")), Descendant(Any, Name("br"))))
                .map(|a| a.parent().unwrap().children().into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .unwrap()
                .parent()
                .select(Txt)
                .iter()
                .map(|a| a.text())
                .collect(),
            None,
        )
        // old
        // ContentType::Text(
        //     d.select(Child(Name("div"), Name("p")))
        //         .map(|a| a.parent().unwrap().children().into_selection())
        //         .max_by(|a, b| a.len().cmp(&b.len()))
        //         .unwrap()
        //         .select(Txt)
        //         .iter()
        //         .map(|a| a.text())
        //         .collect(),
        //     None,
        // )
    })
}
pub fn default_images(page: &Page) -> Images {
    page.doc().map(|d| {
        ContentType::Images(
            d.select(Child(Name("div"), Name("img")))
                .map(|a| a.parent().unwrap().select(Name("img")).into_selection())
                .max_by(|a, b| a.len().cmp(&b.len()))
                .map(|i| {
                    i.iter()
                        .map(|a| {
                            if let Some(n) = a.attr("src") {
                                n.to_owned()
                            } else {
                                a.attr("data-src")
                                    .expect("couldn't find image data-src")
                                    .to_owned()
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
            Some(page.origin()),
        )
    })
}

pub fn realm_next(page: &Page) -> Next {
    page.html.as_ref().and_then(|d| {
        d.split('"')
            .skip_while(|s| !s.contains("nextUrl"))
            .nth(2)
            .map(|s| s.replace('\\', ""))
    })
}
pub fn realm_index(page: &Page) -> Index {
    page.doc().and_then(|d| {
        d.select(And(Name("a"), Attr("href", ())))
            .filter(|a| a.parent().unwrap().text().contains("All chapters are in "))
            .map(|a| a.attr("href").unwrap().to_string())
            .next()
    })
}
pub fn realm_images(page: &Page) -> Images {
    page.html.as_ref().map(|d| {
        ContentType::Images(
            {
                d.split('"')
                    .skip_while(|s| !s.contains("nextUrl"))
                    .skip_while(|s| !s.contains("images"))
                    .skip(2)
                    .step_by(2)
                    .take_while(|s| s.contains("http"))
                    .map(|s| s.replace('\\', ""))
                    .collect()
            },
            Some(page.origin()),
        )
    })
}
