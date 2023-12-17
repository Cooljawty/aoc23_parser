///Contains a set of custom tokens and a tokenizer
pub mod tokenizer { 
    use std::{
        fmt::{self, Formatter, Display},
        io::{self, BufRead},
        error::Error,
        str::FromStr,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        Keyword(&'static str),
        Seperator(&'static str),
        Operator(&'static str),
        Identifier(String),
        Count(u32),
        EndOfInput,
    }

    const KEYWORDS:   &'static [&str] = &["Game"]; 
    const SEPERATORS: &'static [&str] = &[";", ":", ",", "\n"]; 
    const OPERATORS:  &'static [&str] = &["+", "-", "*", "/", "=", "=="]; 

    ///Strictly parses string to single token.
    ///Assumes that tokens are mutualy exlusive
    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok(
                if      let Some(keyword) = KEYWORDS.iter().position(|&k| k == input) { Token::Keyword(KEYWORDS[keyword]) }
                else if let Some(seperator) = SEPERATORS.iter().position(|&s| s == input) { Token::Seperator(SEPERATORS[seperator]) }
                else if let Some(operator) = OPERATORS.iter().position(|&s| s == input) { Token::Operator(OPERATORS[operator]) }
                else if let Ok(num) =  input.parse::<u32>() { Token::Count(num)}  
                else if input.chars().all(|c| c.is_alphanumeric()) { Token::Identifier(input.to_string()) }
                else { return Err(ParseTokenError::InvalidToken(input.to_string())); }
            )
        }
    }

    ///The two possible errors from the tokenizer is an invalid token or some I/O error
    #[derive(Debug)]
    pub enum ParseTokenError {
       InvalidToken(String),
       ParseIntError(std::num::ParseIntError),
       IoError(std::io::Error),
    }
    impl Error for ParseTokenError {}
    impl From<std::num::ParseIntError> for ParseTokenError {
        fn from(err: std::num::ParseIntError) -> ParseTokenError {
            ParseTokenError::ParseIntError(err)
        }    
    }
    impl From<std::io::Error> for ParseTokenError {
        fn from(err: io::Error) -> ParseTokenError {
            ParseTokenError::IoError(err)
        }    
    }
    impl Display for ParseTokenError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::IoError(e) => write!(f, "IO error: {e}"),
                Self::ParseIntError(e) => write!(f, "Failed to convert to integer: {e}"),
                Self::InvalidToken(t) => write!(f, "Cannot parse \"{t}\"  into a token"),
            }
        }
    }

    #[derive(Default)]
    pub struct TokenStream {
        stream: Vec<Token>,
    }
    impl TokenStream {
        pub fn new(tokens: Vec<Token>) -> TokenStream { TokenStream { stream: tokens.into() } }

        ///Takes in input buffer and outputs a stream of tokens
        pub fn tokenize(input: impl BufRead) -> Result<TokenStream, ParseTokenError> {
            let mut token_stack = TokenStream::default();

            for line in input.lines() {
                let mut input = line?;

                let mut curr_token = None;
                let mut marker = 0usize;
                while let Some(input_view) = input.get(0..marker) {
                    if let Some(matching_token) = input_view.trim().parse::<Token>().ok() {
                        curr_token = Some(matching_token);
                        marker += 1;
                    } else if let Some(token) = curr_token {
                        token_stack.stream.push(token);
                        input.replace_range(0..marker-1, "");

                        marker = 0usize;
                        curr_token = None;
                    } else { 
                        marker += 1; 
                    }
                }
                if let Some(token) = curr_token {
                    token_stack.stream.push(token);
                }

                token_stack.stream.push(Token::Seperator("\n"));
            }

            token_stack.stream.push(Token::EndOfInput);
            Ok(token_stack)
        }

    }
    impl Iterator for TokenStream {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.stream.is_empty() {
                Some(self.stream.remove(0))
            } else {
                None
            }
        }
    }
}

///Contains a struct for a stream of tokens and a method to parse and convert to a series of
///instructions.
pub mod parser {
    pub use crate::tokenizer::*;

    impl TokenStream {
        ///Takes in a closure defining parsing rules and returns a set of instructions. Fails if the
        ///rule set returns an error.
        pub fn parse<F, I>(self, mut rule_set: F) -> Result<Vec<I>, Box<dyn std::error::Error>>
        where 
            F: FnMut(&mut Vec<Token>) -> Result<Option<Vec<I>>, ParseTokenError>  
        {
            let mut result = Vec::<Vec<I>>::new();
            let mut buffer = Vec::<Token>::new();
            for token in self {
                buffer.push(token.clone());
                if let Some(instructions) = rule_set(&mut buffer)? {
                    result.push(instructions);
                    buffer.clear();
                }
            }
            let result = result.into_iter().flatten().collect();

            Ok(result)
        }
    }
}
