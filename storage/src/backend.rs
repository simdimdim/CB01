use std::io::Result;

pub mod file;

pub use self::file::*;

#[derive(Debug, Clone)]
pub struct Location(pub url::Url);

pub trait Backend: Sized {
    type Config: Identify;
    type State = ();
    fn save(
        &self, config: Option<Self::Config>,
    ) -> impl std::future::Future<Output = Result<()>> + Send
    where
        Self: Save,
        Self: Send,
        Self: Sync,
        <Self as Backend>::Config: std::marker::Send, {
        async move {
            <Self as Save>::save(
                self,
                <<Self as Backend>::Config as Identify>::id(self).as_str(),
                config,
            )
            .await
        }
    }
    fn load(&mut self, location: url::Url) -> impl std::future::Future<Output = Result<()>> + Send
    where
        Self: Load<Data = Self>,
        Self: Send, {
        async {
            *self = <Self as Load>::load(self, location).await?;
            Ok(())
        }
    }
}
pub trait Save {
    fn save<T: Send>(
        &self, name: &str, config: T,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}
pub trait Load: Sized {
    type Data;
    fn load(&self, _: url::Url) -> impl std::future::Future<Output = Result<Self::Data>> + Send;
    fn load_mut(
        &mut self, _: url::Url,
    ) -> impl std::future::Future<Output = Result<Self::Data>> + Send;
}
pub trait Identify {
    fn id<T>(obj: T) -> String;
}
