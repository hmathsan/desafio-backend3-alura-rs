use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    bank_origin: String,
    agency_origin: String,
    account_origin: String,
    bank_destiny: String,
    agency_destiny: String,
    account_destiny: String,
    value: f32,
    date: String
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
