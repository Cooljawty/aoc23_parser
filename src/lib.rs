pub mod tokenizer { 
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum Token {
        Game,
        Keyword(&'static str),
        Count(i32),
        StartLine,
        EndLine,
    }
    static KEYWORDS: [&'static str;3] = ["red", "green", "blue"]; 

    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok( match input {
                _ if KEYWORDS.iter().any(|&k| k == input)  => Token::Keyword(KEYWORDS.iter().find(|&&k| k == input).unwrap()),
                num if num.parse::<i32>().is_ok() => Token::Count(num.parse::<i32>().unwrap()), 
                _ => { return Err(ParseTokenError{}); },
            })
        }
    }

    pub struct ParseTokenError {}
}
