use pagepal_ui::*;

#[test]
fn book() {
    let (_label, mut book) = Book::open("", std::path::PathBuf::from("."));
    // Test number of pages in a chapter you know the length of
    book.cont_add(
        (0..5).map(|_| Content::Empty).collect::<Vec<Content>>(),
        None,
    );
    book.chap_add_from_parts(None, 2);
    assert_eq!(book.chapter(1).map(Iterator::count), Some(3));
    remove_cont()
}
fn remove_cont() {}
#[test]
fn chapter() {
    let chap = Chapter {
        offset: 1,
        len:    1,
        path:   None,
        src:    None,
        name:   None,
        full:   false,
    };
    assert!(chap.contains(&1));
    shorten_chapter()
}

fn shorten_chapter() {
    let mut c0 = Chapter {
        offset: 39,
        len: 10,
        ..Default::default()
    };
    let mut c1 = Chapter {
        offset: 40,
        len: 10,
        ..Default::default()
    };
    let mut c2 = Chapter {
        offset: 55,
        len: 3,
        ..Default::default()
    };
    let mut c3 = Chapter {
        offset: 45,
        len: 20,
        ..Default::default()
    };
    let mut c4 = Chapter {
        offset: 60,
        len: 10,
        ..Default::default()
    };
    let mut c5 = Chapter {
        offset: 70,
        len: 5,
        ..Default::default()
    };
    let other = Chapter {
        offset: 50,
        len: 10,
        ..Default::default()
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
