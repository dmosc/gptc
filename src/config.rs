use std::env;

pub struct Config {
    pub openai_key: String,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let openai_key = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set.");

        Ok(Self { openai_key })
    }

    pub fn get_openai_key(&self) -> &String {
        &self.openai_key
    }
}
