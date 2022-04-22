use diesel::prelude::*;

use crate::model::transaction::Transaction;

use super::PostgresDatabase;

use crate::schema::transactions::dsl::*;

pub async fn save_transactions(conn: &PostgresDatabase, transactions_vec: Vec<Transaction>) {
    conn.run(|c| diesel::insert_into(transactions).values(transactions_vec).execute(&*c)).await.unwrap();
}