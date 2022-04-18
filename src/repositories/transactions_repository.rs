use crate::model::transaction::Transaction;

use super::get_conn;

pub async fn save_transactions(transaction: &Transaction) {
    let (client, conn) = get_conn().await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    client.query("INSERT INTO transactions
        (id, banco_org, agencia_org, conta_org, banco_dest, agencia_dest, conta_dest, valor, data)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);", 
        &[
            &uuid::Uuid::new_v4().to_string(), 
            &transaction.bank_origin,
            &transaction.agency_origin,
            &transaction.account_origin,
            &transaction.bank_destiny,
            &transaction.agency_destiny,
            &transaction.account_destiny,
            &transaction.value.to_string(),
            &transaction.date
        ]).await.unwrap();
}