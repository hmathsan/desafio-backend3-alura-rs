use std::{fs::File, io::{BufReader, BufRead}, env};

use rocket::{fs::TempFile, http::ContentType, form::{Form, Contextual}, tokio::fs};
use rocket_dyn_templates::Template;

use crate::model::{transaction::Transaction, context::Context};

#[derive(Debug, FromForm)]
pub struct MFD<'v> {
    #[field(validate = ext(ContentType::CSV))]
    csv: TempFile<'v>
}

#[post("/", data = "<data>")]
pub async fn process_uploaded_file<'r>(data: Form<Contextual<'r, MFD<'r>>>) -> Template {    
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
                
                transactions.push(Transaction::new(
                    splitted_line.get(0).unwrap().to_string(),
                    splitted_line.get(1).unwrap().to_string(),
                    splitted_line.get(2).unwrap().to_string(),
                    splitted_line.get(3).unwrap().to_string(),
                    splitted_line.get(4).unwrap().to_string(),
                    splitted_line.get(5).unwrap().to_string(),
                    splitted_line.get(6).unwrap().parse::<f32>().unwrap(),
                    splitted_line.get(7).unwrap().to_string(),
                ));
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

    let context = Context { transactions: file_data };
    Template::render("index", context)
}
