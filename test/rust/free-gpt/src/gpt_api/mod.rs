pub mod text {

    use crate::db::user::History;

    use async_openai::types::CreateChatCompletionRequest;
    use async_openai::{
        types::{
            ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
            CreateChatCompletionRequestArgs, Role,
        },
        Client,
    };
    use futures::StreamExt;
    use std::error::Error;

    pub async fn chat(request: &str, history: &Vec<History>) -> Result<String, Box<dyn Error>> {
        let client = Client::new();

        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();
        for instance in history {
            messages.push(
                ChatCompletionRequestMessageArgs::default()
                    .content(instance.message.clone())
                    .role(Role::User)
                    .build()?,
            );
            messages.push(
                ChatCompletionRequestMessageArgs::default()
                    .content(instance.response.clone())
                    .role(Role::Assistant)
                    .build()?,
            );
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .max_tokens(1024u16)
            .messages(
                [
                    messages.clone(),
                    [ChatCompletionRequestMessageArgs::default()
                        .content(request)
                        .role(Role::User)
                        .build()?]
                    .to_vec(),
                ]
                .concat(),
            )
            .build()?;

        // let request = CreateChatCompletionRequestArgs::default()
        //     .model("gpt-3.5-turbo")
        //     .max_tokens(1024u16)
        //     .messages(
        //         [
        //             messages.clone(),
        //             [ChatCompletionRequestMessageArgs::default()
        //                 .content(request)
        //                 .role(Role::User)
        //                 .build()?]
        //             .to_vec(),
        //         ]
        //         .concat(),
        //     )
        //     .build()?;

        // sync response ... wait for a while possibly
        let data = client.chat().create(request).await?;

        // Streaming, does not work well ...
        // let mut stream = client.chat().create_stream(request).await?;
        //
        // // For reasons not documented in OpenAI docs / OpenAPI spec,
        // // the response of streaming call is different and doesn't include all the same fields.
        //
        // // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
        // //
        // //  Note that stdout is frequently line-buffered by default so it may be necessary
        // //  to use io::stdout().flush() to ensure the output is emitted immediately.
        // //
        // //  The print! macro will lock the standard output on each call.
        // //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
        // //  To avoid this, lock stdout with io::stdout().lock():
        //
        // let mut data = String::new();
        // use std::fmt::Write;
        // while let Some(result) = stream.next().await {
        //     match result {
        //         Ok(response) => {
        //             response.choices.iter().for_each(|chat_choice| {
        //                 if let Some(ref content) = chat_choice.delta.content {
        //                     write!(data, "{}", content).unwrap();
        //                 }
        //             });
        //         }
        //         Err(err) => {
        //             writeln!(data, "error: {err}").unwrap();
        //         }
        //     }
        // }

        let mut response = String::new();
        // println!("\nResponse choices : {}\n", data.choices.len());
        for choice in data.choices {
            if choice.index == 0 {
                response.push_str(&choice.message.content)
            }
        }
        Ok(response)
    }
}
