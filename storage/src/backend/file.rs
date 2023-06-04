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
    async fn save(&self) -> Result<()> { todo!() }
}
impl Load for FileStore {
    type Data = Vec<u8>;

    async fn load(&self, location: url::Url) -> Result<Self::Data> {
        tokio::fs::read(location.path()).await
    }
}
impl Backend for FileStore {
    type Config = FileStoreConfig;
    type State = ();

    // fn config(_: &Self::Config) -> Self { todo!() }
}
