pub mod tokenizer { 
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum Token {
        Game,
        Keyword(String),
        Count(u32),
        Seperator(String),
        StartLine,
        EndLine,
    }

    static KEYWORDS: &[&'static str] = &["red", "green", "blue", "Game"]; 
    static SEPERATORS: &[&'static str] = &[";", ":", ","]; 

    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok( match input {
                keyword   if KEYWORDS.iter().any(|&k| k == input)   => Token::Keyword(keyword.to_string()),
                seperator if SEPERATORS.iter().any(|&s| s == input) => Token::Seperator(seperator.to_string()),
                num if num.parse::<i32>().is_ok() => Token::Count(num.parse::<u32>().unwrap()), 
                _ => { return Err(ParseTokenError{}); },
            })
        }
    }

    pub struct ParseTokenError {}
}
