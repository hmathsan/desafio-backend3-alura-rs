use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub bank_origin: String,
    pub agency_origin: String,
    pub account_origin: String,
    pub bank_destiny: String,
    pub agency_destiny: String,
    pub account_destiny: String,
    pub value: f32,
    pub date: String
}

impl Transaction {
    pub fn new(
        bank_origin: String,
        agency_origin: String,
        account_origin: String,
        bank_destiny: String,
        agency_destiny: String,
        account_destiny: String,
        value: f32,
        date: String
    ) -> Self {
        Self{ 
            bank_origin, 
            agency_origin, 
            account_origin, 
            bank_destiny, 
            agency_destiny, 
            account_destiny, 
            value,
            date
        }
    }
}
