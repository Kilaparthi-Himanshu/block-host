use std::sync::{Arc, Mutex};
use crate::commands::versions_loaders::LoaderSupportCache;

#[derive(Default)]
pub struct AppState {
    pub ping_count: Arc<Mutex<u32>>,
    pub loader_cache: Arc<Mutex<Option<LoaderSupportCache>>>,
}

impl AppState {
    pub fn add(&self) -> () {
        *self.ping_count.lock().unwrap() += 1;
    }
}
