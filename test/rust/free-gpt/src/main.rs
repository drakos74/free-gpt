#[macro_use]
extern crate rocket;

mod db;
mod env;
mod gpt_api;
mod prompts;

use crate::env::file::load;

use crate::db::user::get;
use crate::db::user::put;

use crate::db::user::History;

use crate::gpt_api::text::chat;
use crate::prompts::user::greet;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Request {
    id: u8,
    title: String,
    input: String,
}

#[launch]
fn rocket() -> _ {
    // for local run set the correct directory ...
    // let _ = load("../../gpt.key");
    let _ = load("gpt.key");
    rocket::build().mount("/", routes![get_gpt, post_gpt])
}

#[get("/free-gpt/<user>/<request>")]
async fn get_gpt(user: &str, request: &str) -> String {
    let history = Vec::new();
    let respone = chat(request, &history);
    return respone.await.unwrap_or(format!("error for {}", user));
}

#[post("/free-gpt/v1/<user>", format = "json", data = "<request>")]
async fn post_gpt(user: &str, request: Json<Request>) -> String {
    // first get the users conversation
    let conversation = get(user, request.title.to_string(), request.input.to_string());
    // info!("{:?}", conversation);
    // read the content from the request
    let content = request.input.as_str();
    // apply prompts ...
    let message = greet(user, content);
    // send request ...
    let response = chat(&message, &conversation.messages);
    let response_message = response.await.unwrap_or_else(|e| {
        let error_response = format!("Error : {}", e);
        error_response.to_string()
    });
    let response_history = History {
        message: request.input.to_string(),
        response: response_message.clone(),
    };
    // TODO : get the limit from the user settings
    put(user, 3, conversation, response_history);
    // open response ...
    return response_message.clone();
}
