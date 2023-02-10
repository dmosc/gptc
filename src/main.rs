use std::process::exit;

use clipboard::{ClipboardContext, ClipboardProvider};

mod args;
mod config;
mod gpt_client;
mod logger;

fn main() {
    dotenv::dotenv().ok();

    let args = args::load_args();
    let config = config::Config::new().unwrap();
    let gpt_client = gpt_client::GPTClient::new();
    let response = gpt_client.query(&args, &config).unwrap();
    let reply = response.as_object().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim();

    if reply.is_empty() {
        logger::info("no reply back; try a different wording for the query");
        exit(0);
    }

    logger::info("fetched the following:");

    for line in reply.lines() {
        if line.len() > 0 {
            logger::log(line);
        }
    }

    if args.clipboard {
        let mut clipboard_context = ClipboardContext::new().unwrap();
        clipboard_context.set_contents(reply.to_string()).unwrap();
        logger::info("copied contents to clipboard");
    }
}
