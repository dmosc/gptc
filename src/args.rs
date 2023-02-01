#[derive(Debug, PartialEq)]
pub struct Args {
    query: String,
}

impl Args {
    pub fn new(query: String) -> Self {
        Self { query }
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_args() {
        let query = "hello, world";
        let args = Args::new(String::from(query));

        assert_eq!(args.get_query(), query);
    }
}
