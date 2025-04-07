use icu_segmenter::WordSegmenter;

pub trait Splitter {
    fn split<'a>(&self, input: &'a str) -> Vec<&'a str>;
}

pub struct HATSplitter;

impl Splitter for HATSplitter {
    fn split<'a>(&self, input: &'a str) -> Vec<&'a str> {

        let segmenter = WordSegmenter::new_auto();

        let breakpoints: Vec<usize> = segmenter
            .segment_str(input)
            .collect();

        return breakpoints
            .windows(2)
            .map(|w| &input[w[0]..w[1]])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let splitter = HATSplitter;
        let input = "Hello, world! This is a test.";
        let result = splitter.split(input);
        assert_eq!(result, vec!["Hello,", "world!", "This", "is", "a", "test."]);
    }
}
