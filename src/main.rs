#[macro_use] extern crate rocket;
use serde_json::{Value, json};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

#[get("/json")]
fn json_hello() -> Value {
    json!({"message": "Hello, world!\n"})
}


#[rocket::main]
async fn main() {

    let _ = rocket::build()
            .mount("/", routes![hello, json_hello])
            .launch()
            .await;

}