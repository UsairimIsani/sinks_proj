use crate::store::Store;
// use std::sync::{Arc, RwLock};
use std::sync::Arc;
use tokio::sync::RwLock;

// pub type Stores = Arc<RwLock<Vec<Arc<RwLock<dyn Store>>>>>;
pub type Stores = Arc<RwLock<Vec<Box<dyn Store>>>>;
