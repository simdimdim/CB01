use std::path::PathBuf;

use crate::Content;

#[derive(Debug, Clone)]
pub struct AppData {
    pub current:  Box<Vec<Content>>,
    pub flipped:  bool,
    pub reversed: bool,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            current:  Box::new(vec![
                Content::Image {
                    pb:  PathBuf::from("library/01.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/02.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/03.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/04.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/05.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/06.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/07.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/08.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/09.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/10.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/11.jpg"),
                    src: None,
                },
                Content::Image {
                    pb:  PathBuf::from("library/12.jpg"),
                    src: None,
                },
                /* Content::Image {
                 *     pb:  PathBuf::from("library/13.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/14.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/15.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/16.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/17.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/18.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/19.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/20.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/21.jpg"),
                 *     src: None,
                 * },
                 * Content::Image {
                 *     pb:  PathBuf::from("library/22.jpg"),
                 *     src: None,
                 * }, */
            ]),
            flipped:  false,
            reversed: false,
        }
    }
}
