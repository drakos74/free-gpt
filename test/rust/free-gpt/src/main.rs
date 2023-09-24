#[macro_use]
extern crate rocket;

mod gpt_api;
mod prompts;

use crate::gpt_api::text::call;
use crate::prompts::user::greet;

use std::io::{stdout, Write};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Request {
    name: String,
    id: u8,
    input: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_gpt, post_gpt])
}

#[get("/free-gpt/<user>/<request>")]
async fn get_gpt(user: &str, request: &str) -> String {
    let respone = call(request);
    return respone.await.unwrap_or("error".to_string());
}

#[post("/free-gpt/v1/<user>", format = "json", data = "<request>")]
async fn post_gpt(user: &str, request: Json<Request>) -> String {
    let content = request.input.as_str();

    let message = greet(user, content);

    let response = call(&message);

    return response.await.unwrap_or("error".to_string());
}
