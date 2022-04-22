use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::import_history;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="import_history"]
pub struct ImportHistory {
    pub id: String,
    pub data_transacoes: String,
    pub data_importacao: String
}

impl ImportHistory {
    pub fn new(data_transacoes: String, data_importacao: String) -> Self {
        Self{ id: Uuid::new_v4().to_string(), data_transacoes, data_importacao }
    }
}