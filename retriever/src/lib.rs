#![feature(associated_type_defaults)]
#![feature(iter_advance_by)]

pub mod extractor;
pub mod page;
pub mod presets;
pub mod retriever;

use page::ContentType;

type Title = Option<String>;
type Index = Option<String>;
type Next = Option<String>;
type Links = Option<Vec<String>>;
type Text = Option<ContentType>;
type Images = Option<ContentType>;
