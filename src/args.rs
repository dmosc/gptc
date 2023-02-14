use clap::Parser;

#[derive(Debug, clap::Parser, PartialEq)]
#[command(author, version, about)]
pub struct Args {
    /// Prompt sent to GPT model.
    pub prompt: String,

    /// Copy output to clipboard.
    #[arg(short, long)]
    pub clipboard: bool,
}

pub fn load_args() -> Args {
    Args::parse()
}
