#![deny(missing_docs)]
//! 
use rocket::serde::json::{Json, Value, json};

mod routes;



#[macro_use]
extern crate rocket;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::hello])
}
