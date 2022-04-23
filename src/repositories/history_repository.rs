use diesel::prelude::*;

use crate::model::import_history::ImportHistory;
use crate::schema::import_history::dsl::*;

use super::PostgresDatabase;

pub async fn save_import_history(conn: &PostgresDatabase, history: Vec<ImportHistory>) {
    conn.run(|c| diesel::insert_into(import_history).values(history).execute(&*c) ).await.unwrap();
}

pub async fn get_all_history_by_user(conn: &PostgresDatabase, user_id_string: String) -> Vec<ImportHistory> {
    conn.run(|c| import_history.filter(user_id.eq(user_id_string)).load::<ImportHistory>(&*c).unwrap()).await
}