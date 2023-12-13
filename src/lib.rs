///Contains a set of custom tokens and a tokenizer
pub mod tokenizer { 
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Keyword(String),
        Count(u32),
        Seperator(String),
        Identifier(String),
    }

    const KEYWORDS: &'static [&str] = &["red", "green", "blue", "Game"]; 
    const SEPERATORS: &'static [&str] = &[";", ":", ","]; 

    ///Strictly parses string to single token.
    ///Assumes that tokens are mutualy exlusive
    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok( match input {
                keyword   if KEYWORDS.iter().any(|&k| k == input)   => Token::Keyword(keyword.to_string()),
                seperator if SEPERATORS.iter().any(|&s| s == input) => Token::Seperator(seperator.to_string()),
                identifier if identifier.chars().all(|c| c.is_alphanumeric())=> Token::Identifier(identifier.to_string()),
                num if num.parse::<i32>().is_ok() => Token::Count(num.parse::<u32>().unwrap()), 
                _ => { return Err(ParseTokenError{}); },
            })
        }
    }
    pub struct ParseTokenError {}

    ///Takes in input string and outputs a stream of tokens
    pub fn tokenize(input: &String) -> Vec<Token> {
        let mut input = input.clone();
        let mut token_stack = Vec::<Token>::new();

        let mut curr_token = None;
        let mut marker = 0usize;
        while let Some(input_view) = input.get(0..marker) {
            if let Some(matching_token) = input_view.trim().parse::<Token>().ok() {
                curr_token = Some(matching_token);
                marker += 1;
            } else if let Some(token) = curr_token {
                token_stack.push(token);
                input.replace_range(0..marker-1, "");

                marker = 0usize;
                curr_token = None;
            } else { 
                marker += 1; 
            }
        }
        if let Some(token) = curr_token {
            token_stack.push(token);
        }

        token_stack
    }
}
