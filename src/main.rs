mod models;
mod routes;
mod validators;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::validate])
}
