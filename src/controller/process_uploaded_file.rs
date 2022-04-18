use std::{fs::File, io::{BufReader, BufRead}, env};

use chrono::NaiveDateTime;
use rocket::{fs::TempFile, http::ContentType, form::{Form, Contextual}, tokio::fs};
use rocket_dyn_templates::Template;

use crate::{model::{transaction::Transaction, context::Context}, repositories::{transactions_repository::save_transactions, history_repository::save_import_history}};

#[derive(Debug, FromForm)]
pub struct MFD<'v> {
    #[field(validate = ext(ContentType::CSV))]
    csv: TempFile<'v>
}

#[post("/", data = "<data>")]
pub async fn process_uploaded_file<'r>(data: Form<Contextual<'r, MFD<'r>>>) -> Template {    
    let mut history = vec![];
    let file_data: Option<Vec<Transaction>> = match data.into_inner().value {
        Some(data) => {
            let path = format!("{}\\temp.csv", env::current_dir().unwrap().to_string_lossy());

            let mut csv_file = data.csv;
            csv_file.copy_to(&path).await.unwrap();

            let csv_file_str = File::open(&path).unwrap();
            let reader = BufReader::new(csv_file_str);
            
            let mut transactions = vec![];

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

                history.push(save_import_history(date.format("%d/%m/%Y").to_string()).await);
                save_transactions(&transaction).await;

                transactions.push(transaction);
            }

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
