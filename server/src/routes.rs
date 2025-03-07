use rocket::serde::json::{Json, Value, json};

#[get("/hello/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> Value {
    json!(
        {
            "name": name, 
            "age": age
        })
}