use reqwest::{blocking::Client, Error};
use serde_json::json;

use crate::{args, config};

pub struct GPTClient {
    http_client: Client,
}

impl GPTClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub fn query(
        &self,
        args: &args::Args,
        config: &config::Config,
    ) -> Result<serde_json::Value, Error> {
        match self
            .http_client
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
                "prompt": args.prompt
            }))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", config.get_openai_key()),
            )
            .send()
        {
            Ok(response) => response.json::<serde_json::Value>(),
            Err(error) => return Err(error),
        }
    }
}
