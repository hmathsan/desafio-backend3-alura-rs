use chrono::{NaiveDateTime, Utc, DateTime};
use rocket::{request::{FromRequest, Outcome}, Request};

use crate::vars::default_user_login_timeout;

pub struct AuthValidation;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthValidation {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private("user_id") {
            Some(_user_id) => {
                match request.cookies().get_private("user_login_time") {
                    Some(user_login_time) => {
                        let user_login_time: DateTime<Utc> = DateTime::from_utc(
                            NaiveDateTime::parse_from_str(&user_login_time.value(), "%Y-%m-%d %H:%M:%S").unwrap(),
                            Utc
                        );
                        let now = Utc::now();
                        
                        let diff = now - user_login_time;

                        if diff.num_minutes() > default_user_login_timeout() as i64 {
                            println!("User authentication expired");
                            Outcome::Forward(())
                        } else {
                            println!("User is authenticated");
                            Outcome::Success(AuthValidation)
                        }
                    },
                    None => {
                        Outcome::Forward(())
                    },
                }
            },
            None => Outcome::Forward(()),
        }
    }

}