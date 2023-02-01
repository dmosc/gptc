use std::env;

use gptc::load_args;

pub mod args;

fn main() {
    let args = load_args(env::args());
    println!("{:?}", args);
}
