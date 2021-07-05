use pagepal_ui::*;

#[tokio::test]
async fn fetch() {
    let p = "https://readmanganato.com/manga-fz982434/chapter-16".into();
    let r = Retriever::default();
    let page = r.get(p).await;
    let next = r.next(&page).await;
    let index = r.index(&next.unwrap()).await;
    let _ = r.links(&index).await;
}

#[test]
fn book() {
    let mut book = Book::open("One Piece");
    // Test number of pages in a chapter you know the length of
    book.cont_add((0..5).map(|_| Box::new(vec![])).collect(), None);
    book.chap_add(None, 2);
    assert_eq!(book.chapter(1).map(Iterator::count), Some(3));
    remove_cont()
}
fn remove_cont() {}
#[test]
fn chapter() { shorten_chapter() }

fn shorten_chapter() {
    let mut c0 = Chapter {
        offset: 39,
        len:    10,
        name:   None,
    };
    let mut c1 = Chapter {
        offset: 40,
        len:    10,
        name:   None,
    };
    let mut c2 = Chapter {
        offset: 55,
        len:    3,
        name:   None,
    };
    let mut c3 = Chapter {
        offset: 45,
        len:    20,
        name:   None,
    };
    let mut c4 = Chapter {
        offset: 60,
        len:    10,
        name:   None,
    };
    let mut c5 = Chapter {
        offset: 70,
        len:    5,
        name:   None,
    };
    let other = Chapter {
        offset: 50,
        len:    10,
        name:   None,
    };
    assert_eq!(c0.shorten(other.range()), 10);
    assert_eq!(c1.shorten(other.range()), 9);
    assert_eq!(c2.shorten(other.range()), 0);
    assert_eq!(c3.shorten(other.range()), 9);
    assert_eq!(c4.shorten(other.range()), 9);
    assert_eq!(c5.shorten(other.range()), 5);
    assert_eq!(c0.start(), 39);
    assert_eq!(c1.start(), 40);
    assert_eq!(c2.start(), 55);
    assert_eq!(c3.start(), 45);
    assert_eq!(c4.start(), 50);
    assert_eq!(c5.start(), 59)
}
