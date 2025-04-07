pub trait Splitter {
    fn split<'a>(&self, input: &'a str) -> Vec<&'a str>;
}

pub struct WhitespaceSplitter;

impl Splitter for WhitespaceSplitter {
    fn split<'a>(&self, input: &'a str) -> Vec<&'a str> {
        input.split_whitespace().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let splitter = WhitespaceSplitter;
        let input = "Hello, world! This is a test.";
        let result = splitter.split(input);
        assert_eq!(result, vec!["Hello,", "world!", "This", "is", "a", "test."]);
    }
}
