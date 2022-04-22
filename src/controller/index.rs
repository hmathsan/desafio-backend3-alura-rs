use rocket_dyn_templates::Template;

use crate::{model::context::Context, repositories::{history_repository::get_all_history, PostgresDatabase}};

#[get("/")]
pub async fn index(conn: PostgresDatabase) -> Template {
    let history = get_all_history(&conn).await;

    let context = Context { transactions: Vec::new(), history };
    Template::render("index", &context)
}