use rocket::{fairing::{Fairing, Info, Kind, self}, Rocket, Build};
use diesel::prelude::*;
use argonautica::Hasher;
use rocket_sync_db_pools::database;

use crate::{vars::{default_user_email, default_user_password, secret_key}, model::user::User};

use crate::schema::users::dsl::*;

pub struct RepositoryFairing;

pub mod transactions_repository;
pub mod history_repository;
pub mod user_repository;

#[database("postgres")]
pub struct PostgresDatabase(pub diesel::PgConnection);

#[rocket::async_trait]
impl Fairing for RepositoryFairing {
    fn info(&self) -> Info {
        Info {
            name: "Liftoff Db Check",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        println!("Checking if default user is in the database.");
        let db = PostgresDatabase::get_one(&rocket).await.unwrap();

        let default_password = default_user_password();

        let default_user = db.run(|conn| users
            .filter(email.eq(default_user_email()))
            .load::<User>(&*conn)
            .unwrap()).await;

        if default_user.len() <= 0 {
            println!("Default user doesn't exist, creating.");
            let mut hasher = Hasher::default();

            let password = hasher.with_password(default_password)
                .with_secret_key(secret_key())
                .hash().unwrap();

            let default_user = User {
                id: uuid::Uuid::new_v4().to_string(),
                email: default_user_email(),
                nome: "Admin".to_string(),
                senha: password
            };

            db.run(|conn| diesel::insert_into(users).values(default_user).get_result::<User>(&*conn))
                .await.expect("Error creating default user.");

            println!("Default user created.");
        } else {
            println!("Default user already exists on the database.");
        }

        Ok(rocket)
    }
}