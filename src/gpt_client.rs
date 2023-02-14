use crate::{args, config, logger};

use http_cache_reqwest;
use reqwest;
use reqwest_middleware;
use serde_json::json;

pub struct GPTClient {
    http_client: reqwest_middleware::ClientWithMiddleware,
}

impl GPTClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
                .with(http_cache_reqwest::Cache(http_cache_reqwest::HttpCache {
                    mode: http_cache_reqwest::CacheMode::Default,
                    manager: http_cache_reqwest::CACacheManager::default(),
                    options: None,
                }))
                .build(),
        }
    }

    pub async fn async_query(
        &self,
        args: &args::Args,
        config: &config::Config,
    ) -> reqwest_middleware::Result<serde_json::Value> {
        logger::info("waiting for reply...");
        let response = self
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
            .await?;

        Ok(response.json::<serde_json::Value>().await?)
    }
}
