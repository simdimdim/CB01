pub mod library;
pub mod retriever;
pub mod ui;

pub use self::{library::*, retriever::*, ui::*};

pub static APP_NAME: &str = "pagepal";
