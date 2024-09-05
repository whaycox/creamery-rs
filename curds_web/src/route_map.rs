use curds_core::io::FileSystem;
use tokio::sync::Notify;
use super::*;

pub struct CurdsWebRouteMap {
    configuration: Arc<SiteConfiguration>,
    debouncer: Debouncer<ReadDirectoryChangesWatcher, FileIdMap>,
}

impl CurdsWebRouteMap {
    pub async fn new<TFileSystem : FileSystem + Send + Sync + 'static>(file_system: Arc<TFileSystem>, root_path: &Path) -> CurdsWebResult<Self> {
        let root_regex = Arc::new(Regex::new(&format!("^{}(.*)$", regex::escape(root_path.canonicalize().unwrap().to_str().unwrap()))).unwrap());
        let entries = Self::route_directory(file_system.clone(), root_path, &root_regex).await?;
        let map = Arc::new(RwLock::new(entries.into_iter().collect()));
        let configuration = Arc::new(SiteConfiguration::new(root_path, root_regex, map));

        let notify = Arc::new(Notify::new());
        tokio::spawn(Self::update_map(file_system.clone(), configuration.clone(), notify.clone()));

        let mut debouncer = new_debouncer(Duration::from_secs(2), None, Self::file_update_handler(notify.clone())).unwrap();
        debouncer.watcher().watch(root_path, RecursiveMode::Recursive).unwrap();
        debouncer.cache().add_root(root_path, RecursiveMode::Recursive);

        Ok(Self {
            configuration,
            debouncer: debouncer,
        })
    }
    fn update_map<TFileSystem : FileSystem + Send + Sync + 'static>(file_system: Arc<TFileSystem>, configuration: Arc<SiteConfiguration>, notify: Arc<Notify>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>  {
        Box::pin(async move {
            loop {
                notify.notified().await;
                
                match Self::route_directory(file_system.clone(), &configuration.root, &configuration.regex).await {
                    Ok(entries) => {
                        let mut writable_map = configuration.map.write().unwrap();
                        *writable_map = entries.into_iter().collect();
                        log::info!("Updated file map");
                    },
                    Err(error) => log::error!("Error when updating map: {error}"),
                }
            }
        })
    }
    fn route_directory<'a, TFileSystem : FileSystem + Send + Sync + 'static>(file_system: Arc<TFileSystem>, directory: &'a Path, root: &'a Regex) -> Pin<Box<dyn Future<Output = CurdsWebResult<Vec<(UriPath, String)>>> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut entries: Vec<(UriPath, String)> = vec![(generate_target(&directory, &root)?, expand_path(&directory, true))];
            let mut read_iterator = file_system.list_files(directory).await.unwrap();
            while let Some(entry) = read_iterator.next_entry().await.unwrap() {
                let path = entry.path();
                if entry.metadata().await.unwrap().is_dir() {
                    entries.extend(Self::route_directory(file_system.clone(), &path, root).await?);
                }
                else {
                    entries.push((generate_target(&path, &root)?, expand_path(&path, false)));
                }
            }

            Ok(entries)
        })
    }
    fn file_update_handler(sender: Arc<Notify>) -> impl Fn(DebounceEventResult) {
        move |_: DebounceEventResult| {
            let sender = sender.clone();
            sender.notify_one();
        }
    }

    pub fn retrieve(&self, target: &UriPath) -> Option<String> { 
        if let Some(path) = self.configuration.map.read().unwrap().get(target) {
            return Some(path.clone());
        }
        None
     }
}


fn generate_target(path: &Path, root: &Regex) -> CurdsWebResult<UriPath> {
    let mut target = expand_path(path, false);
    target = root.captures(&target)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .replace("\\", "/");
    if target.len() == 0 {
        target = "/".to_owned();
    }
    let mut target = target.into_bytes();

    UriPath::parse(&mut target)
}
fn expand_path(path: &Path, expand_index: bool) -> String {
    let mut expanded = path.canonicalize().unwrap();
    if expand_index && expanded.is_dir() {
        expanded.push("index.html");
    }

    return expanded
        .to_str()
        .unwrap()
        .to_owned()
}