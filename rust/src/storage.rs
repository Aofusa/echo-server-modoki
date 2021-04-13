use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

pub type Values = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub msg: String
}

#[derive(Clone)]
pub struct Storage {
    pub storage: Arc<RwLock<Values>>
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

