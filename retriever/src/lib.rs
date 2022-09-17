#![feature(associated_type_defaults)]
#![feature(iter_advance_by)]

pub mod hounds;
pub mod page;

use page::ContentType;

type Title = Option<String>;
type Index = Option<String>;
type Next = Option<String>;
type Links = Option<Vec<String>>;
type Text = Option<ContentType>;
type Images = Option<ContentType>;
