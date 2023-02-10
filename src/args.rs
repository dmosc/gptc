use clap::Parser;

#[derive(Debug, Parser, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Prompt sent to GPT model.
    #[arg(short, long)]
    pub prompt: String,
}

pub fn load_args() -> Args {
    Args::parse()
}
