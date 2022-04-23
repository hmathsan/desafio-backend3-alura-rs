use dotenv::dotenv;

use std::env::var;

pub fn secret_key() -> String {
    dotenv().ok();

    var("SECRET_KEY").expect("SECRET_KEY is not set")
}

pub fn default_user_email() -> String {
    dotenv().ok();

    var("DEFAULT_USER_EMAIL").expect("DEFAULT_USER_EMAIL is not set")
}

pub fn default_user_password() -> String {
    dotenv().ok();

    var("DEFAULT_USER_PASSWORD").expect("DEFAULT_USER_PASSWORD is not set")
}

pub fn default_user_login_timeout() -> usize {
    dotenv().ok();

    var("USER_LOGIN_TIMEOUT").expect("USER_LOGIN_TIMEOUT is not set").parse::<usize>().unwrap()
}