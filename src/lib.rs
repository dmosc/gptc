mod args;

pub fn load_args(mut args: impl Iterator<Item = String>) -> Result<args::Args, &'static str> {
    // Skip first argument containing binary path.
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("expecting query string"),
    };

    Ok(args::Args::new(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_args_all_args() {
        let args = vec![String::from("hello, world")];
        let loaded_args = load_args(args.into_iter()).unwrap();

        assert_eq!(loaded_args, args::Args::new(String::from("hello, world")));
    }
}
