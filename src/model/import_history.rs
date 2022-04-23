use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::import_history;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations, PartialEq)]
#[diesel(belongs_to(User))]
#[table_name="import_history"]
pub struct ImportHistory {
    pub id: String,
    pub user_id: String,
    pub data_transacoes: String,
    pub data_importacao: String
}

impl ImportHistory {
    pub fn new(user_id: String, data_transacoes: String, data_importacao: String) -> Self {
        Self{ id: Uuid::new_v4().to_string(), user_id, data_transacoes, data_importacao }
    }
}