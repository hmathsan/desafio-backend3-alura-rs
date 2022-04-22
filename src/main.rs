#[macro_use]
extern crate diesel;

use repositories::{PostgresDatabase, RepositoryFairing};
use rocket_dyn_templates::Template;

mod repositories;
mod controller;
mod schema;
mod model;
mod vars;

use controller::{index::{index, login}, process_uploaded_file::process_uploaded_file, import_history::import_history};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDatabase::fairing())
        .attach(RepositoryFairing)
        .mount("/", routes![index, login])
        .mount("/process_data", routes![process_uploaded_file])
        .mount("/import_transaction", routes![import_history])
        .attach(Template::fairing())
}