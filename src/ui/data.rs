use crate::Content;

#[derive(Debug)]
pub struct AppData {
    pub current:  Box<Vec<Content>>,
    pub flipped:  bool,
    pub reversed: bool,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            current:  Box::new(vec![
                Content::Image("library/01.jpg".into()),
                Content::Image("library/02.jpg".into()),
                Content::Image("library/03.jpg".into()),
                Content::Image("library/04.jpg".into()),
                Content::Image("library/05.jpg".into()),
                Content::Image("library/06.jpg".into()),
                Content::Image("library/07.jpg".into()),
                Content::Image("library/08.jpg".into()),
                Content::Image("library/09.jpg".into()),
                Content::Image("library/10.jpg".into()),
                Content::Image("library/11.jpg".into()),
                Content::Image("library/12.jpg".into()),
                Content::Image("library/13.jpg".into()),
                Content::Image("library/14.jpg".into()),
                Content::Image("library/15.jpg".into()),
                Content::Image("library/16.jpg".into()),
                Content::Image("library/17.jpg".into()),
                Content::Image("library/18.jpg".into()),
                Content::Image("library/19.jpg".into()),
                Content::Image("library/20.jpg".into()),
                /*  Content::Image("library/21.jpg".into()),
                 * Content::Image("library/22.jpg".into()), */
            ]),
            flipped:  false,
            reversed: false,
        }
    }
}
