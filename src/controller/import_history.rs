use rocket::{request::FlashMessage, http::CookieJar};
use rocket_dyn_templates::Template;

#[get("/import_transaction")]
pub async fn import_history(
    flash: Option<FlashMessage<'_>>,
    cookies: &CookieJar<'_>
) -> Template {
    println!("{} {}", flash.unwrap().message(), cookies.get_private("user_jwt").unwrap());

    todo!()
}