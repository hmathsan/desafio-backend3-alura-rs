use rocket_dyn_templates::Template;

use crate::{model::context::Context, repositories::history_repository::get_all_history};

#[get("/")]
pub async fn index() -> Template {
    let history = get_all_history().await;

    let context = Context { transactions: Vec::new(), history };
    Template::render("index", &context)
}