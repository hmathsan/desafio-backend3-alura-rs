use chrono::Utc;
use rocket::{http::{CookieJar, Cookie}, form::Form, request::FlashMessage, response::{Flash, Redirect}};
use rocket_dyn_templates::Template;

use crate::{repositories::{user_repository::find_user_by_email, PostgresDatabase}, model::context::LoginContext};

#[get("/")]
pub async fn login_screen(
    flash: Option<FlashMessage<'_>>
) -> Template {
    println!("Entrou /login");
    let error_message = match flash {
        Some(message) => message.message().to_string(),
        None => String::new(),
    };

    let is_error = error_message == "O email ou senha estão incorretos.";

    let context = LoginContext{ is_error, error_message };
    Template::render("login/login_index", &context)
}

#[derive(FromForm, Debug)]
pub struct UserLoginForm<'r> {
    pub email: &'r str,
    pub password: &'r str
}

#[post("/", data = "<data>")]
pub async fn validate_login(data: Form<UserLoginForm<'_>>, cookies: &CookieJar<'_>, db_conn: PostgresDatabase) -> Flash<Redirect> {
    match find_user_by_email(&db_conn, String::from(data.email)).await {
        Some(user) => {
            if user.is_password_valid(String::from(data.password)) {
                cookies.add_private(Cookie::new("user_id", user.id));
                cookies.add_private(Cookie::new("user_login_time", Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()));
                return Flash::success(Redirect::to(uri!("/import_transaction")), "User logged successfully")
            } else {
                return Flash::error(Redirect::to(uri!("/login")), "O email ou senha estão incorretos.")
            }
        },
        None => return Flash::error(Redirect::to(uri!("/login")), "O email ou senha estão incorretos.")
    }
}