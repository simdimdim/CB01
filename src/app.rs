pub mod library;
pub mod misc;
pub mod retriever;
pub mod ui;

pub use self::{library::*, misc::*, retriever::*, ui::*};

pub static APP_NAME: &str = "pagepal";
