#[macro_use] extern crate rocket;
use serde_json::{Value, json};
use rocket::serde::json::Json;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

#[get("/json")]
fn json_hello() -> Value {
    json!({"message": "Hello, world!"})
}

#[get("/people")]
fn view_people_data() -> Value {
    json!([{"id" : 1, "name": "Sid Krishnan", "email" : "sid@sidkrishnan.com"}, {"id" : 2, "name": "Siddharth Krishnan", "email" : "not available"}])
}

#[get("/people/<id>")]
fn view_people_data_by_id(id: i32) -> Value {
    let people = view_people_data();

    let people_array = people.as_array().unwrap();

    for person in people_array {
        if person["id"] == id {
            return person.clone();
        }
    }

    json!({
        "error": "Person not found"
    })
}


#[post("/people", format = "json", data = "<person>")]
fn create_people_data(person: Json<Value>) -> Value {
    person.into_inner()
}



#[rocket::main]
async fn main() {

    let _ = rocket::build()
            .mount("/", routes![hello, json_hello, view_people_data, view_people_data_by_id, create_people_data])
            .launch()
            .await;

}