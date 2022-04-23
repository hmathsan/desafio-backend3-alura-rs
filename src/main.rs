#[macro_use]
extern crate diesel;

use repositories::{PostgresDatabase, RepositoryFairing};
use rocket_dyn_templates::Template;

mod repositories;
mod controller;
mod schema;
mod guards;
mod model;
mod vars;

use controller::{
    index::index, 
    process_uploaded_file::{process_uploaded_file, process_uploaded_file_redirect}, 
    import_history::{import_history, import_history_redirect}, 
    login::{login_screen, validate_login, login_redirect}
};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDatabase::fairing())
        .attach(RepositoryFairing)
        .mount("/", routes![index])
        .mount("/login", routes![login_redirect, login_screen, validate_login])
        .mount("/process_data", routes![process_uploaded_file, process_uploaded_file_redirect])
        .mount("/import_transaction", routes![import_history, import_history_redirect])
        .attach(Template::fairing())
}