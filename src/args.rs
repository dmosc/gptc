use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
pub struct Args {
    /// Prompt sent to GPT model.
    #[arg(short, long)]
    pub prompt: String,
}
