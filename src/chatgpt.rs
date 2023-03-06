use std::env;

use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE}};
use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
enum GPTRole {
    System,
    User,
    Assistant
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
struct GPTMessage {
    role: GPTRole,
    content: String,
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
struct GPTRequest {
    model: String,
    messages: Vec<GPTMessage>,
}

fn new_init_message() -> Vec<GPTMessage> {
    vec![GPTMessage {
        role: GPTRole::System,
        content: std::fs::read_to_string("init.txt").unwrap()
    }]
}

pub async fn response_from_chatgpt(content: String) -> String {
    let message = GPTMessage {
        role: GPTRole::User,
        content,
    };

    let mut messages = new_init_message();
    messages.push(message);

    let client = Client::new();
    let res = client.post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, 
            format!("Bearer {}", env::var("OPENAI_KEY").expect("OPENAI_KEY not set") ) 
        )
        .header(CONTENT_TYPE, "application/json")
        .body(
            serde_json::to_string(&GPTRequest {
                model: "gpt-3.5-turbo".to_string(),
                messages: messages.clone(),
            }).unwrap()
        )
        .send().await
        .unwrap().json::<Value>().await.unwrap();

    let response = res["choices"][0]["message"]["content"].as_str().unwrap().to_string();


    response
}
