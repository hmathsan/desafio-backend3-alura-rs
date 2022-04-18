use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportHistory {
    pub transaction_date: String,
    pub import_date: String
}

impl ImportHistory {
    pub fn new(transaction_date: String, import_date: String) -> Self {
        Self{ transaction_date, import_date }
    }
}