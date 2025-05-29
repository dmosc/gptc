use colored::Colorize;

pub fn log(contents: &str) {
    println!("{}", contents.white().bold());
}

pub fn info(contents: &str) {
    println!("{} {}", "[info]:".blue().bold(), contents.blue().bold());
}

pub fn warn(contents: &str) {
    println!("{} {}", "[warn]:".yellow().bold(), contents.yellow().bold());
}
