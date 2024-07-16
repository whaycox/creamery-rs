use super::*;
use tokio::{fs::File, io::Result};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[whey_mock]
#[allow(async_fn_in_trait)]
pub trait FileSystem {
    async fn read_string(&self, path: &str) -> Result<String>;
    async fn write_bytes(&self, path: &str, bytes: &[u8]) -> Result<()>;
}

pub struct AsyncFileSystem;

impl FileSystem for AsyncFileSystem {
    async fn read_string(&self, path: &str) -> Result<String> {
        log::info!("Reading {}", path);
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        Ok(contents)
    }

    async fn write_bytes(&self, path: &str, bytes: &[u8]) -> Result<()> {
        log::info!("Writing {} bytes to {}", bytes.len(), path);
        let mut file = File::create(path).await?;
        file.write_all(bytes).await?;

        Ok(())
    }
}