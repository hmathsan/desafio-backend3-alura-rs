use diesel::prelude::*;

use super::PostgresDatabase;

use crate::model::user::User;
use crate::schema::users::dsl::*;

pub async fn find_user_by_email(db_conn: &PostgresDatabase, user_email: String) -> Option<User> {
    match db_conn.run(|conn| users.filter(email.eq(user_email)).load::<User>(&*conn)).await {
        Ok(user_data) => {
            if user_data.len() > 1 {
                //TODO: use warn from env_logger
                println!("More than one user with the same email registered.")
            }
            match user_data.get(0) {
                Some(user_data) => Some(user_data.clone()),
                None => None,
            }
        },
        Err(_) => None,
    }
}