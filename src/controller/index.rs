use rocket_dyn_templates::Template;

use crate::model::context::Context;

#[get("/")]
pub fn index() -> Template {
    let context = Context { transactions: Vec::new(), history: Vec::new() };
    Template::render("index", &context)
}