use std::path::PathBuf;
use super::*;

pub struct SiteConfiguration {
    pub root: PathBuf,
    pub regex: Arc<Regex>,
    pub map: Arc<RwLock<HashMap<UriPath, String>>>,
}

impl SiteConfiguration {
    pub fn new(root: &Path, regex: Arc<Regex>, map: Arc<RwLock<HashMap<UriPath, String>>>) -> Self {
        Self {
            root: root.to_owned(),
            regex,
            map,
        }
    }
}