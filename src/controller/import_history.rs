use rocket::{http::CookieJar, response::Redirect};
use rocket_dyn_templates::Template;

use crate::{
    repositories::{
        history_repository::get_all_history_by_user, 
        PostgresDatabase
    }, 
    model::context::Context, 
    guards::auth_guard::AuthValidation
};

#[get("/")]
pub async fn import_history(
    cookies: &CookieJar<'_>,
    db_conn: PostgresDatabase,
    _auth_guard: AuthValidation
) -> Template {
    let user_id = String::from(cookies.get_private("user_id").unwrap().value());

    let history = get_all_history_by_user(&db_conn, user_id).await;

    let context = Context { transactions: vec![], history };
    Template::render("import_transactions/import_transaction_index", &context)
}

#[get("/", rank = 2)]
pub async fn import_history_redirect() -> Redirect {
    Redirect::to(uri!("/"))
}