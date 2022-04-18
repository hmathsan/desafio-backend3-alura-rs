use serde::{Serialize, Deserialize};

use super::transaction::Transaction;

#[derive(Serialize, Deserialize)]
pub struct Context {
    pub transactions: Vec<Transaction>
}