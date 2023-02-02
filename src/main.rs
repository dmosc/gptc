use reqwest::blocking::Client;
use serde_json::json;
use std::env;

use gptc::load_args;

mod args;
mod config;

fn main() {
    let config = config::Config::new().unwrap();
    let args = load_args(env::args()).unwrap();
    let http_client = Client::new();
    let response = http_client
        .post("https://api.openai.com/v1/completions")
        .json(&json!({
            "top_p": 1,
            "stop": "```",
            "temperature": 0,
            "suffix": "\n```",
            "max_tokens": 1000,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "model": "text-davinci-003",
            "prompt": args.get_query()
        }))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", config.get_openai_key()),
        )
        .send()
        .unwrap();
    println!("{:?}", response.text());
}
