use diesel::prelude::*;

use crate::model::import_history::ImportHistory;

use super::PostgresDatabase;

use crate::schema::import_history::dsl::*;

pub async fn save_import_history(conn: &PostgresDatabase, history: Vec<ImportHistory>) {
    conn.run(|c| diesel::insert_into(import_history).values(history).execute(&*c) ).await.unwrap();
}

pub async fn get_all_history(conn: &PostgresDatabase) -> Vec<ImportHistory> {
    conn.run(|c| import_history.load::<ImportHistory>(&*c).unwrap()).await
}