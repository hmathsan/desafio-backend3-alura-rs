#[macro_use]
extern crate diesel;

use repositories::{PostgresDatabase, RepositoryFairing};
use rocket_dyn_templates::Template;

mod repositories;
mod controller;
mod schema;
mod model;
mod vars;

use controller::{index::index, process_uploaded_file::process_uploaded_file};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PostgresDatabase::fairing())
        .attach(RepositoryFairing)
        .mount("/", routes![index, process_uploaded_file])
        .attach(Template::fairing())
}