use super::*;

#[whey_mock]
pub trait FileRouter {
    fn route_request<'a>(&self, target: &'a UriPath) -> Pin<Box<dyn Future<Output = CurdsWebResult<Vec<u8>>> + Send + Sync + 'a>>;
}

pub struct CurdsWebFileRouter<TFileSystem> {
    file_system: Arc<TFileSystem>,
    map: Arc<RwLock<CurdsWebRouteMap>>,
}

pub type ProductionFileRouter = CurdsWebFileRouter<AsyncFileSystem>;
impl ProductionFileRouter {
    pub async fn new() -> CurdsWebResult<Self> {
        let file_system = Arc::new(AsyncFileSystem);
        let root_path = Path::new("site");
        let map = Arc::new(RwLock::new(CurdsWebRouteMap::new(file_system.clone(), &root_path).await?));
        
        Ok(Self {
            file_system,
            map,
        })
    }
}

impl<TFileSystem> FileRouter for CurdsWebFileRouter<TFileSystem> where 
TFileSystem : FileSystem + Send + Sync + 'static {
    fn route_request<'a>(&self, target: &'a UriPath) -> Pin<Box<dyn Future<Output = CurdsWebResult<Vec<u8>>> + Send + Sync + 'a>> {
        let file_system = self.file_system.clone();
        let map = self.map.clone();
        Box::pin(async move {
            let mut path = None;
            {
                let lock = map.read().unwrap();
                if let Some(entry) = lock.retrieve(&target) {
                    path = Some(entry);
                }
            }
            if let Some(file_path) = path {
                if let Ok(file_bytes) = file_system.read_bytes(Path::new(&file_path)).await {
                    return Ok(file_bytes);
                }
            }

            Err(CurdsWebError::FileNotFound(target.to_owned()))            
        })
    }
}