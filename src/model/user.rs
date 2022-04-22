use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub nome: String,
    pub senha: String
}