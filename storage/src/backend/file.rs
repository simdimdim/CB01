use crate::*;
use std::{io::Result, path::PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileStore {
    location: PathBuf,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileStoreConfig {
    pub location: PathBuf,
}
impl Save for FileStore {
    async fn save<T>(&self, _name: &str, _config: T) -> Result<()>
    where
        <Self as Backend>::Config: Identify, {
        todo!()
    }
}
impl Load for FileStore {
    type Data = Vec<u8>;

    async fn load(&self, location: url::Url) -> Result<Self::Data> {
        tokio::fs::read(location.path()).await
    }

    async fn load_mut(&mut self, _location: url::Url) -> Result<Self::Data>
    where
        Self: Sized, {
        // *self = tokio::fs::read(location.path()).await?;
        // Ok(self)
        todo!()
    }
}
impl Backend for FileStore {
    type Config = FileStoreConfig;
    type State = ();
}

impl Identify for FileStoreConfig {
    fn id<T>(_: T) -> String { todo!() }
}
