use super::*;
use tokio::{fs::File, io::Result};

#[whey_mock]
#[allow(async_fn_in_trait)]
pub trait FileSystem {
    async fn read(&self, path: &str) -> Result<File>;
    async fn write(&self, path: &str) -> Result<File>;
}

pub struct AsyncFileSystem;

impl FileSystem for AsyncFileSystem {
    async fn read(&self, path: &str) -> Result<File> {
        File::open(path).await
    }

    async fn write(&self, path: &str) -> Result<File> {
        File::create(path).await
    }
}