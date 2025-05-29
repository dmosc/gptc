use clipboard::ClipboardProvider;
use std::process;
use tokio; // Needed to bring the ClipboardProvider into context.

mod args;
mod config;
mod gpt_client;
mod logger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> reqwest_middleware::Result<()> {
    dotenv::dotenv().ok();

    let args = args::load_args();
    let config = config::Config::new().unwrap();
    let gpt_client = gpt_client::GPTClient::new();
    let response = gpt_client.async_query(&args, &config).await?;
    let reply = response.as_object().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim();

    if reply.is_empty() {
        logger::info("no reply back; try a different wording for the query");
        process::exit(0);
    }

    logger::info("fetched the following:");

    for line in reply.lines() {
        if line.len() > 0 {
            logger::log(line);
        }
    }

    if args.clipboard {
        let mut clipboard_context = clipboard::ClipboardContext::new().unwrap();
        clipboard_context.set_contents(reply.to_string()).unwrap();
        logger::info("copied contents to clipboard");
    }

    Ok(())
}
