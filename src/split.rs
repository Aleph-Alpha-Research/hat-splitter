use once_cell::sync::Lazy;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

enum Token {
    Word(String),
    Punctuation(String),
    Whitespace(String),
    Space(String),
}

impl Token {
    fn inner(self) -> String {
        match self {
            Token::Word(s) | Token::Punctuation(s) | Token::Whitespace(s) | Token::Space(s) => s,
        }
    }
}

pub trait Splitter {
    // At some point it would be great to do this without allocations...
    //fn split<'a>(&self, input: &'a str) -> Vec<&'a str>;

    /// Splits a string into words.
    fn split(&self, text: &str) -> Vec<String>;

    /// Splits a string into words and limits the size of each word to `max_bytes_per_word`. As
    /// this function enforces a byte limit, it may split unicode characters. That is, this
    /// function does not guarantee that the resulting byte arrays are valid UTF-8.
    fn split_with_limit(&self, text: &str, max_bytes_per_word: usize) -> Vec<Vec<u8>>;
}

pub struct HATSplitter;

impl Default for HATSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl HATSplitter {
    pub fn new() -> Self {
        Self
    }

    fn unicode_word_split(input: &str) -> Vec<&str> {
        input.split_word_bounds().collect::<Vec<&str>>()
    }

    fn split_at_matches<'a>(s: &'a str, re: &Regex) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut word_start = 0;

        for regex_match in re.find_iter(s) {
            let match_start = regex_match.start();

            // We can unwrap here as we assume the regex match points to a valid UTF-8 character
            let word_end = match_start + s[match_start..].chars().next().unwrap().len_utf8();

            result.push(&s[word_start..word_end]);
            word_start = word_end;
        }

        if word_start < s.len() {
            result.push(&s[word_start..s.len()]);
        }

        result
    }

    fn split_camel_case(s: &str) -> Vec<&str> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\p{Ll})(\p{Lu})").unwrap());
        Self::split_at_matches(s, &RE)
    }

    fn split_punctuation(s: &str) -> Vec<&str> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\p{P}").unwrap());
        Self::split_at_matches(s, &RE)
    }

    fn combine_spaces(strings: Vec<&str>) -> Vec<String> {
        strings.into_iter().fold(Vec::new(), |mut acc, s| {
            if s == " " {
                // If we have a space and the last element is also spaces, append to it
                if let Some(last) = acc.last_mut() {
                    if last.chars().all(|c| c == ' ') {
                        last.push(' ');
                        return acc;
                    }
                }
            }
            // Otherwise add as a new element
            acc.push(s.to_string());
            acc
        })
    }

    // This function does its best to avoid splitting unicode characters, but in some cases it has
    // no choice (e.g., if max_bytes < 4 and an emoji comes in).
    fn split_long_words(strings: Vec<String>, max_bytes: usize) -> Vec<Vec<u8>> {
        if max_bytes == 0 {
            panic!("max_bytes must be greater than 0");
        }
        strings.into_iter().fold(Vec::new(), |mut result, string| {
            let bytes = string.as_bytes();
            if bytes.len() <= max_bytes {
                result.push(bytes.to_vec());
                return result;
            }

            let mut start_byte = 0;
            while start_byte < bytes.len() {
                let end_byte = std::cmp::min(start_byte + max_bytes, bytes.len());

                // Backtrack to find a valid UTF-8 boundary
                let end = (start_byte + 1..=end_byte)
                    .rev()
                    .find(|&i| string.is_char_boundary(i))
                    .unwrap_or(end_byte); // Fall back to end_byte if no boundary found

                result.push(bytes[start_byte..end].to_vec());
                start_byte = end;
            }
            result
        })
    }

    /// The Lexer takes a string and splits it into logical tokens.
    fn lex(s: &str) -> Vec<Token> {
        static WHITESPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());
        static PUNCTUATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\p{P}$").unwrap());

        let words = Self::combine_spaces(
            Self::unicode_word_split(s)
                .iter()
                .flat_map(|s| Self::split_punctuation(s))
                .flat_map(|s| Self::split_camel_case(s))
                .collect::<Vec<&str>>(),
        );

        words
            .into_iter()
            .map(|s| {
                if s == " " {
                    Token::Space(s)
                } else if WHITESPACE_RE.is_match(s.as_str()) {
                    Token::Whitespace(s)
                } else if PUNCTUATION_RE.is_match(s.as_str()) {
                    Token::Punctuation(s)
                } else {
                    Token::Word(s)
                }
            })
            .collect()
    }

    /// The Parser takes tokens and groups them into a string split.
    fn parse(tokens: Vec<Token>) -> Vec<String> {
        let groups = tokens
            .into_iter()
            .fold(Vec::<Vec<Token>>::new(), |mut groups, token| {
                let should_append_to_last_group = |last_group: &Vec<Token>, token: &Token| {
                    matches!(
                        (last_group.last(), token),
                        (Some(Token::Space(_)), Token::Word(_))
                            | (
                                Some(Token::Space(_) | Token::Word(_) | Token::Punctuation(_)),
                                Token::Punctuation(_),
                            )
                    )
                };

                if let Some(last_group) = groups.last_mut() {
                    if should_append_to_last_group(last_group, &token) {
                        last_group.push(token);
                        return groups;
                    }
                }

                groups.push(vec![token]);
                groups
            });

        // Concatenate groups
        groups
            .into_iter()
            .map(|group| group.into_iter().map(Token::inner).collect())
            .collect()
    }
}

impl Splitter for HATSplitter {
    fn split(&self, input: &str) -> Vec<String> {
        Self::parse(Self::lex(input))
    }

    fn split_with_limit(&self, input: &str, max_bytes: usize) -> Vec<Vec<u8>> {
        Self::split_long_words(Self::parse(Self::lex(input)), max_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static STRANGE_STUFF: &str = "𓀀✨𝒜𝓁𝑔𝑜𝓇𝒾𝓉𝒽𝓂 شْء 你好吗 こんにちは 안녕하세요 𞤢𞤭𞤤 𝔽(λx.𝑥²) 🤖🍕⟨𝛴, 𝜋⟩ 🜚 𝔽↦𝑒ⁿω₀📡;𝑧𝑎<𝔱𝓇𝑢∃>🛠️ҀЋހ±(Δ𝓧) 乁( •_• )ㄏ   ⿰木日👾";

    #[test]
    fn it_works() {
        let result = HATSplitter::new().split("Hello, world!");

        assert_eq!(result, vec!["Hello,", " world!"]);
    }

    #[test]
    fn it_handles_empty_input() {
        let result = HATSplitter::new().split("");

        assert!(result.is_empty());
    }

    #[test]
    fn it_splits_camel_case() {
        let result = HATSplitter::new().split("howAreYou");

        assert_eq!(result, vec!["how", "Are", "You"]);
    }

    #[test]
    fn it_splits_snake_case() {
        let result = HATSplitter::new().split("how_are_you");

        assert_eq!(result, vec!["how_", "are_", "you"]);
    }

    #[test]
    fn it_limits_word_size() {
        let result = HATSplitter::new().split_with_limit("verylongword", 10);

        assert_eq!(result, vec![b"verylongwo".to_vec(), b"rd".to_vec()]);
    }

    #[test]
    fn it_splits_large_unicode_characters() {
        let result = HATSplitter::new().split_with_limit("🌝", 2);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn it_does_not_split_unicode_where_possible() {
        // This is one word with a 2-byte 'ü' starting at byte offset 1. We hope that the splitter
        // preserves this character by splitting into three parts instead of two.
        let result = HATSplitter::new().split_with_limit("für", 2);

        assert_eq!(
            result,
            vec![b"f".to_vec(), "ü".as_bytes().to_vec(), b"r".to_vec()]
        );
    }

    #[test]
    #[should_panic]
    fn it_handles_zero_max_bytes() {
        HATSplitter::new().split_with_limit("abc", 0);
    }

    #[test]
    fn it_handles_strange_stuff() {
        HATSplitter::new().split_with_limit(STRANGE_STUFF, 100);
    }

    #[test]
    fn it_is_causal() {
        let max_chunk_size = 1024;
        let splitter = HATSplitter::new();

        let full_split = splitter.split_with_limit(STRANGE_STUFF, max_chunk_size);

        for (i, _) in STRANGE_STUFF.char_indices() {
            let prefix = &STRANGE_STUFF[..i];
            let partial_split = splitter.split_with_limit(prefix, max_chunk_size);

            for (full_word, partial_word) in full_split.iter().zip(partial_split.iter()) {
                assert_eq!(&full_word[..partial_word.len()], partial_word);
            }
        }
    }
}
