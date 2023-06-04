use std::io::Result;

pub mod file;

pub use self::file::*;

#[derive(Debug, Clone)]
pub struct Location(pub url::Url);

pub trait Backend: Sized {
    type Config;
    type State = ();
    async fn save<T>(&self) -> Result<()>
    where
        Self: Save, {
        <Self as Save>::save(self).await
    }
    async fn load(&mut self, location: url::Url) -> Result<()>
    where
        Self: Load<Data = Self>, {
        *self = <Self as Load>::load(self, location).await?;
        Ok(())
    }
}
pub trait Save {
    async fn save(&self) -> Result<()>;
}
pub trait Load {
    type Data;
    async fn load(&self, _: url::Url) -> Result<Self::Data>
    where
        Self: Sized;
}
