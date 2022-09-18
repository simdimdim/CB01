use crate::{
    page::{ContentType, Page},
    Images,
    Index,
    Links,
    Next,
    Text,
    Title,
};
use select::predicate::{Any, Child, Descendant, Name, Or, Text as Txt};

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
    None
}
pub fn default_links(page: &Page) -> Links {
    page.doc().map(|d| {
        d.select(Descendant(
            Name("div"),
            Or(Name("p"), Or(Name("table"), Name("ul"))),
        ))
        .map(|a| a.select(Name("a")).into_selection())
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .filter_map(|a| a.attr("href"))
        .map(|a| a.to_string())
        .map(Into::into)
        .collect()
    })
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
                .unwrap()
                .iter()
                .map(|a| {
                    if let Some(n) = a.attr("src") {
                        n.to_owned()
                    } else {
                        a.attr("data-src")
                            .expect("couldn't find image data-src")
                            .to_owned()
                    }
                })
                .collect(),
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
