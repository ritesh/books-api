use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// A very simple database
pub type Db = Arc<Mutex<Vec<Book>>>;

//FixedResponse returns a welcome message
#[derive(Serialize, Deserialize, Debug)]
pub struct FixedResponse {
    pub status: String,
    pub remote_address: String,
    pub hostname: String,
}

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
}

// The query parameters for list books.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
