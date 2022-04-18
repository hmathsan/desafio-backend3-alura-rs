use chrono::{self, Local};

use crate::model::import_history::ImportHistory;

use super::get_conn;

pub async fn save_import_history(date: String) -> ImportHistory {
    let (client, conn) = get_conn().await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    let current_date_time = Local::now().format("%d/%m/%Y - %T");

    client.query("INSERT INTO import_history 
        (id, data_transacoes, data_importacao)
        VALUES($1, $2, $3)",
        &[
            &uuid::Uuid::new_v4().to_string(),
            &date,
            &current_date_time.to_string()
        ]).await.unwrap();

    ImportHistory::new(date, current_date_time.to_string())
}

pub async fn get_all_history() -> Vec<ImportHistory> {
    let (client, conn) = get_conn().await.unwrap();

    let mut history: Vec<ImportHistory> = vec![];

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    for row in &client.query("SELECT * FROM import_history", &[]).await.unwrap() {
        history.push(ImportHistory { transaction_date: row.get(1), import_date: row.get(2) })
    }

    history
}