use argonautica::Verifier;
use serde::{Serialize, Deserialize};

use crate::{schema::users, vars::secret_key};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub nome: String,
    pub senha: String
}

impl User {
    pub fn is_password_valid(&self, password: String) -> bool {
        let mut verifier = Verifier::default();
        verifier
            .with_hash(self.senha.clone())
            .with_password(password)
            .with_secret_key(secret_key())
            .verify()
            .unwrap()
    }
}