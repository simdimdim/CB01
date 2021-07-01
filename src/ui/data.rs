use crate::Content;

#[derive(Debug)]
pub struct AppData {
    pub current:  Vec<Content>,
    pub flipped:  bool,
    pub reversed: bool,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            current:  vec![
                Content::Image("library/1.jpg".into()),
                Content::Image("library/2.jpg".into()),
                Content::Image("library/3.jpg".into()),
            ],
            flipped:  false,
            reversed: false,
        }
    }
}
