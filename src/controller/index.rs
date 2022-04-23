use chrono::{NaiveDateTime, Utc, DateTime};
use rocket::{http::{CookieJar, Cookie}, response::{Redirect, Flash}};

use crate::vars::default_user_login_timeout;

#[get("/")]
pub async fn index(
    cookies: &CookieJar<'_>
) -> Flash<Redirect> {
    //TODO: use flash message to display a error message

    match cookies.get_private("user_id").map(|cookie| format!("{}", cookie.value())) {
        Some(_user_id) => {
            match cookies.get_private("user_login_time").map(|cookie| format!("{}", cookie.value())) {
                Some(user_login_time) => {
                    let user_login_time: DateTime<Utc> = DateTime::from_utc(
                        NaiveDateTime::parse_from_str(&user_login_time, "%Y-%m-%d %H:%M:%S").unwrap(),
                        Utc
                    );
                    let now = Utc::now();

                    let diff = now - user_login_time;

                    if diff.num_minutes() > default_user_login_timeout() as i64 {
                        return Flash::error(Redirect::to(uri!("/login")), "Usuário não logado ou login expirado. Por favor faça o login.")
                    } else {
                        return Flash::success(Redirect::to(uri!("/import_transaction")), "User already logged in.")
                    }
                },
                None => {
                    cookies.remove_private(Cookie::named("user_id"));
                    return Flash::error(Redirect::to(uri!("/login")), "user_login_time cookie is not set, redoing login.")
                },
            }            
        },
        None => return Flash::error(Redirect::to(uri!("/login")), "User not logged in."),
    };
}