use rocket_dyn_templates::Template;

mod controller;
mod model;

use controller::{index::index, process_uploaded_file::process_uploaded_file};


#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, process_uploaded_file])
        .attach(Template::fairing())
}