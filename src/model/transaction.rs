use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::schema::transactions;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[table_name="transactions"]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub banco_org: String,
    pub agencia_org: String,
    pub conta_org: String,
    pub banco_dest: String,
    pub agencia_dest: String,
    pub conta_dest: String,
    pub valor: f32,
    pub data: String
}

impl Transaction {
    pub fn new(
        user_id: String,
        banco_org: String,
        agencia_org: String,
        conta_org: String,
        banco_dest: String,
        agencia_dest: String,
        conta_dest: String,
        valor: f32,
        data: String
    ) -> Self {
        Self{
            id: Uuid::new_v4().to_string(),
            user_id,
            banco_org, 
            agencia_org, 
            conta_org, 
            banco_dest, 
            agencia_dest, 
            conta_dest, 
            valor,
            data
        }
    }
}
