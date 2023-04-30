#[macro_use]
extern crate rocket;

use std::error::Error;
use std::io::{stdout, Write};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt;
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
    let respone = call_gpt(request).await.unwrap_or("error".to_string());
    return respone;
}

#[post("/free-gpt/v1/<user>", format = "json", data = "<request>")]
async fn post_gpt(user: &str, request: Json<Request>) -> String {
    let respone = call_gpt(request.input.as_str())
        .await
        .unwrap_or("error".to_string());
    return respone;
}

async fn call_gpt(request: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(1024u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content(request)
            .role(Role::User)
            .build()?])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    // For reasons not documented in OpenAI docs / OpenAPI spec,
    // the response of streaming call is different and doesn't include all the same fields.

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    let mut data = String::new();
    use std::fmt::Write;
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        write!(data, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(data, "error: {err}").unwrap();
            }
        }
    }

    Ok(data)
}
