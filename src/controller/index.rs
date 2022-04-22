use rocket::{http::{CookieJar, Cookie}, form::Form, response::{Redirect, Flash}, request::FlashMessage};
use rocket_dyn_templates::Template;

use crate::{model::context::{Context, LoginContext}, repositories::{history_repository::get_all_history, PostgresDatabase, user_repository::find_user_by_email}};

use crate::controller::import_history::rocket_uri_macro_import_history;

#[get("/")]
pub async fn index(
    cookies: &CookieJar<'_>, 
    flash: Option<FlashMessage<'_>>
) -> Template {
    //TODO: use flash message to display a error message

    // let user_jwt = match cookies.get_private("user_jwt").map(|cookie| format!("{}", cookie.value())) {
    //     Some(jwt) => {
    //         todo!()
    //     },
    //     None => todo!(),
    // };

    let error_message = match flash {
        Some(message) => message.message().to_string(),
        None => String::new(),
    };

    let context = LoginContext{ error_message };
    Template::render("login/login_index", &context)
}

#[derive(FromForm, Debug)]
pub struct UserLoginForm<'r> {
    pub email: &'r str,
    pub password: &'r str
}

#[post("/", data = "<data>")]
pub async fn login(data: Form<UserLoginForm<'_>>, cookies: &CookieJar<'_>, db_conn: PostgresDatabase) -> Flash<Redirect> {
    match find_user_by_email(&db_conn, String::from(data.email)).await {
        Some(user) => {
            if user.is_password_valid(String::from(data.password)) {
                cookies.add_private(Cookie::new("user_jwt", "jwt"));
                return Flash::success(Redirect::to(uri!(import_history)), "User logged successfully")
            } else {
                return Flash::error(Redirect::to(uri!(index)), "O email ou senha estão incorretos.")
            }
        },
        None => return Flash::error(Redirect::to(uri!("/")), "O email ou senha estão incorretos.")
    }
}