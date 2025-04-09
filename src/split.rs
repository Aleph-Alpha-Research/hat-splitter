use std::sync::LazyLock;

use icu_segmenter::WordSegmenter;
use once_cell::sync::Lazy;
use regex::Regex;

// Note: we could also try `new_auto` which uses a LSTM (we should figure out which is better)
static WORD_SEGMENTER: LazyLock<WordSegmenter> = LazyLock::new(WordSegmenter::new_dictionary);

#[derive(Clone)]
enum Token {
    Word(String),
    Punctuation(String),
    Whitespace(String),
    Space(String),
}

pub trait Splitter {
    // At some point it would be great to do this without allocations...
    //fn split<'a>(&self, input: &'a str) -> Vec<&'a str>;
    fn split(&self, input: &str) -> Vec<String>;
}

pub struct HATSplitter;

impl Default for HATSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl HATSplitter {
    pub fn new() -> Self {
        HATSplitter
    }

    fn _unicode_word_split(input: &str) -> Vec<&str> {
        let breakpoints: Vec<usize> = WORD_SEGMENTER.segment_str(input).collect();

        breakpoints.windows(2).map(|w| &input[w[0]..w[1]]).collect()
    }

    fn _split_camel_case(s: &str) -> Vec<&str> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\p{Ll})(\p{Lu})").unwrap());
        let mut indices = RE.find_iter(s).map(|m| m.start() + 1).collect::<Vec<_>>();

        indices.insert(0, 0);
        indices.push(s.len());

        indices.windows(2).map(|w| &s[w[0]..w[1]]).collect()
    }

    fn _concatenate_spaces(strings: Vec<&str>) -> Vec<String> {
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

    fn _lexer(s: &str) -> Vec<Token> {
        let words = HATSplitter::_unicode_word_split(s);

        let words = words
            .iter()
            .flat_map(|s| HATSplitter::_split_camel_case(s))
            .collect::<Vec<&str>>();

        let words = HATSplitter::_concatenate_spaces(words.clone());

        static WHITESPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());
        static PUNCTUATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\p{P}$").unwrap());

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

    fn _parser(tokens: Vec<Token>) -> Vec<String> {
        let groups = tokens
            .iter()
            .fold(Vec::<Vec<Token>>::new(), |mut groups, token| {
                match token {
                    Token::Whitespace(_) => {
                        // Create a separate group for whitespace
                        groups.push(vec![token.clone()]);
                    }
                    Token::Space(_) => {
                        // Start new group with space
                        groups.push(vec![token.clone()]);
                    }
                    Token::Word(_) => {
                        // Append to current group if last token is a space, otherwise start new group
                        if let Some(last_group) = groups.last_mut() {
                            if let Some(Token::Space(_)) = last_group.last() {
                                last_group.push(token.clone());
                                return groups;
                            }
                        }
                        groups.push(vec![token.clone()]);
                    }
                    Token::Punctuation(_) => {
                        // Append to current group if last token is a word, punctuation or space, otherwise start new group
                        if let Some(last_group) = groups.last_mut() {
                            if let Some(last_token) = last_group.last() {
                                if matches!(
                                    last_token,
                                    Token::Space(_) | Token::Word(_) | Token::Punctuation(_)
                                ) {
                                    last_group.push(token.clone());
                                    return groups;
                                }
                            }
                        }
                        groups.push(vec![token.clone()]);
                    }
                }
                groups
            });

        // Concatenate groups
        groups
            .into_iter()
            .map(|group| {
                group.into_iter().fold(String::new(), |mut acc, token| {
                    match token {
                        Token::Word(s) => acc.push_str(&s),
                        Token::Punctuation(s) => acc.push_str(&s),
                        Token::Whitespace(s) => acc.push_str(&s),
                        Token::Space(s) => acc.push_str(&s),
                    }
                    acc
                })
            })
            .collect()
    }
}

impl Splitter for HATSplitter {
    fn split(&self, input: &str) -> Vec<String> {
        let tokens = HATSplitter::_lexer(input);
        HATSplitter::_parser(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let splitter = HATSplitter;
        let input = "Hello, world!";
        let result = splitter.split(input);
        assert_eq!(result, vec!["Hello,", " world!"]);
    }
}
