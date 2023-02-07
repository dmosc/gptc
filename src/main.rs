use clap::Parser;

mod args;
mod config;
mod gpt_client;

fn main() {
    let args = args::Args::parse();
    let config = config::Config::new().unwrap();
    let gpt_client = gpt_client::GPTClient::new();
    let response = gpt_client.query(&args, &config).unwrap();
    let reply = response.as_object().unwrap()["choices"][0]["text"]
        .as_str()
        .unwrap()
        .trim();

    for line in reply.lines() {
        if line.len() > 0 {
            println!("{}", line);
        }
    }
}
