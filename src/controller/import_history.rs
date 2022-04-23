use rocket::http::CookieJar;
use rocket_dyn_templates::Template;

use crate::{repositories::{history_repository::get_all_history_by_user, PostgresDatabase}, model::context::Context};

#[get("/")]
pub async fn import_history(
    cookies: &CookieJar<'_>,
    db_conn: PostgresDatabase
) -> Template {
    let user_id = String::from(cookies.get_private("user_id").unwrap().value());

    let history = get_all_history_by_user(&db_conn, user_id).await;

    let context = Context { transactions: vec![], history };
    Template::render("import_transactions/import_transaction_index", &context)
}