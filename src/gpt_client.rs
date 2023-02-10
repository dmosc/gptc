use reqwest::{blocking::Client, Error};
use serde_json::json;

use crate::{args, config, logger};

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
        logger::info("waiting for reply...");
        match self
            .http_client
            .post("https://api.openai.com/v1/completions")
            .json(&json!({
                "temperature": 0,
                "max_tokens": 1000,
                "presence_penalty": 0,
                "frequency_penalty": 0,
                "model": "text-davinci-003",
                // TODO: Test out prefix query alternatives.
                // The current prefix *seems* to work in all cases
                // but is important to test a wide range of programming
                // languages, technologies, and operating systems to make sure.
                // Example queries that have been tested so far:
                //
                // gptc -p "linux script to install a binary in bin/ directory."
                // gptc -p "npm command to install dependencies."
                // gptc -p "dnf centos command to list installed dependencies."
                // gptc -p "dnf centos command to list package or group of packages installed; then grep response to find tmux dependency."
                "prompt": format!("reply only with code: {}", args.prompt)
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
