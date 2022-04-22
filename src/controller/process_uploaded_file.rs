use std::{fs::File, io::{BufReader, BufRead}, env};

use chrono::{NaiveDateTime, Local};
use rocket::{fs::TempFile, http::ContentType, form::{Form, Contextual}, tokio::fs};
use rocket_dyn_templates::Template;

use crate::{model::{transaction::Transaction, context::Context, import_history::ImportHistory}, repositories::{transactions_repository::save_transactions, history_repository::save_import_history, PostgresDatabase}};

#[derive(Debug, FromForm)]
pub struct MFD<'v> {
    #[field(validate = ext(ContentType::CSV))]
    csv: TempFile<'v>
}

#[post("/", data = "<data>")]
pub async fn process_uploaded_file<'r>(data: Form<Contextual<'r, MFD<'r>>>, conn: PostgresDatabase) -> Template {
    let mut history = vec![];
    let file_data: Option<Vec<Transaction>> = match data.into_inner().value {
        Some(data) => {
            let path = format!("{}\\temp.csv", env::current_dir().unwrap().to_string_lossy());

            let mut csv_file = data.csv;
            csv_file.copy_to(&path).await.unwrap();

            let csv_file_str = File::open(&path).unwrap();
            let reader = BufReader::new(csv_file_str);
            
            let mut transactions = vec![];

            let current_date_time = Local::now().format("%d/%m/%Y - %T").to_string();

            for line in reader.lines() {
                let line = line.unwrap();
                let splitted_line: Vec<&str> = line.split(",").collect();
                
                let date = NaiveDateTime::parse_from_str(
                    &splitted_line.get(7).unwrap().to_string().replace("T", " "),
                    "%Y-%m-%d %H:%M:%S"
                ).unwrap();

                let transaction = Transaction::new(
                    splitted_line.get(0).unwrap().to_string(),
                    splitted_line.get(1).unwrap().to_string(),
                    splitted_line.get(2).unwrap().to_string(),
                    splitted_line.get(3).unwrap().to_string(),
                    splitted_line.get(4).unwrap().to_string(),
                    splitted_line.get(5).unwrap().to_string(),
                    splitted_line.get(6).unwrap().parse::<f32>().unwrap(),
                    date.format("%d/%m/%Y %T").to_string(),
                );

                history.push(
                    ImportHistory::new(date.format("%d/%m/%Y").to_string(), current_date_time.clone()));

                transactions.push(transaction);
            }

            save_transactions(&conn, transactions.clone()).await;
            save_import_history(&conn, history.clone()).await;

            fs::remove_file(&path).await.unwrap();

            Some(transactions)
        },
        None => None,
    };

    let file_data = match file_data {
        Some(transactions) => {transactions},
        None => Vec::new()
    };

    let context = Context { transactions: file_data, history };
    Template::render("index", context)
}
