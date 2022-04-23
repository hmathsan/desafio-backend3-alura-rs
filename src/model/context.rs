use serde::{Serialize, Deserialize};

use super::{transaction::Transaction, import_history::ImportHistory};

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub transactions: Vec<Transaction>,
    pub history: Vec<ImportHistory>
}

#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    pub is_error: bool,
    pub error_message: String
}