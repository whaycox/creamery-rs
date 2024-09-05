use super::*;
use tokio::{fs::File, io::Result};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs::ReadDir;
use std::path::Path;
use std::pin::Pin;
use std::future::Future;

#[whey_mock]
pub trait FileSystem {
    fn list_files<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<ReadDir>> + Send + Sync + 'a>>;
    fn read_string<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'a>>;
    fn read_bytes<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + Sync + 'a>>;
    fn write_bytes<'a>(&self, path: &'a Path, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'a>>;
}

pub struct AsyncFileSystem;

impl FileSystem for AsyncFileSystem {
    fn list_files<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<ReadDir>> + Send + Sync + 'a>> {
        Box::pin(async move {
            tokio::fs::read_dir(path).await
        })
    }

    fn read_string<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<String>> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
    
            Ok(contents)
        })
    }
    
    fn read_bytes<'a>(&self, path: &'a Path) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut file = File::open(path).await?;
            let mut contents: Vec<u8> = Vec::new();
            file.read_to_end(&mut contents).await?;
    
            Ok(contents)
        })
    }

    fn write_bytes<'a>(&self, path: &'a Path, bytes: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut file = File::create(path).await?;
            file.write_all(bytes).await?;
    
            Ok(())
        })
    }
}