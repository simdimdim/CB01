use crate::*;

pub use self::file::*;

pub trait Store {
    type Backend: Backend = FileStore;
    type BackendConfig = <Self::Backend as Backend>::Config where <Self as store::Store>::Backend: backend::Backend;
}
pub struct DefaultStore {}
impl Store for DefaultStore {}
