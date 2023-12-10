pub mod tokenizer { 
    use std::str::FromStr;
    pub enum Token {
        Color(&'static str),
        Count(i32),
    }
    static KEYWORDS: [&'static str;3] = ["red", "green", "blue"]; 

    impl FromStr for Token {
        type Err = ParseTokenError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            Ok( match input {
                _ if KEYWORDS.iter().any(|&k| k == input)  => Token::Color(KEYWORDS.iter().find(|&&k| k == input).unwrap()),
                num if num.parse::<i32>().is_ok() => Token::Count(num.parse::<i32>().unwrap()), 
                _ => { return Err(ParseTokenError{}); },
            })
        }
    }

    pub struct ParseTokenError {}
}
