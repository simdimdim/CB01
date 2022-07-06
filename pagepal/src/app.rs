pub mod bimap;
pub mod library;
pub mod retriever;
pub mod ui;

pub use self::{bimap::*, library::*, retriever::*, ui::*};

pub static APP_NAME: &str = "pagepal";
