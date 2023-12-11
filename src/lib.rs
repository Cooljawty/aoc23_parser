pub mod tokenizer { 
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum Token {
        Game,
        Keyword(&'static str),
        Count(u32),
        Seperator(&'static str),
        StartLine,
        EndLine,
    }
    static KEYWORDS: &[&'static str] = &["red", "green", "blue", "Game"]; 
    static SEPERATORS: &[&'static str] = &[";", ":", ","]; 

    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok( match input {
                _ if KEYWORDS.iter().any(|&k| k == input)  => Token::Keyword(KEYWORDS.iter().find(|&&k| k == input).unwrap()),
                _ if SEPERATORS.iter().any(|&k| k == input)  => Token::Seperator(SEPERATORS.iter().find(|&&s| s == input).unwrap()),
                num if num.parse::<i32>().is_ok() => Token::Count(num.parse::<u32>().unwrap()), 
                _ => { return Err(ParseTokenError{}); },
            })
        }
    }

    pub struct ParseTokenError {}
}
