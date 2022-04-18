#[macro_use] extern crate dotenv_codegen;

use repositories::RepositoryFairing;
use rocket_dyn_templates::Template;

mod repositories;
mod controller;
mod model;

use controller::{index::index, process_uploaded_file::process_uploaded_file};


#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RepositoryFairing)
        .mount("/", routes![index, process_uploaded_file])
        .attach(Template::fairing())
}