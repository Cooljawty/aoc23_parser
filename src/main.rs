use reqwest::header::{ USER_AGENT, COOKIE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input_src = "https://adventofcode.com/2023/day/1/input";

    let puzzle_input = get_input(puzzle_input_src)?;

    let answer = puzzle_input.iter().map(|value| parse_input(value));
    print!("{}", answer.fold(0, |a,n| a + n));
    Ok(())
}

fn pop_number(value: &mut String) -> Option<usize> {
    static TOKENS: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 

    let mut substr = String::new();
    while !value.is_empty() {
        if !value.is_empty() {substr.push(value.remove(0)); }

        //Try getting numerics first
        let digits: Vec<&str> = substr.matches(char::is_numeric).collect();
        if !digits.is_empty() { return Some(digits.first()?.parse::<usize>().ok()?) }

        for token in &TOKENS {
            let digits: Vec<&str> = substr.matches(token).collect();
            if !digits.is_empty() { 
                return match digits.first() {
                    Some(digit) => TOKENS.iter().position(|t| t == digit),
                    None => None,
                }
            }
        }
    }

    None
}

fn parse_input(value: &String) -> u32 {
    let mut value = value.clone();

    let mut digits = Vec::<u32>::new();
    while !value.is_empty() { match pop_number(&mut value) {
        Some(num) => { digits.push(num as u32); }, //Should work as long as usize is atleast 4bits
        None => { break; }
    }}

    if digits.len() == 1 { digits.push(digits[0]) };

    digits.remove(0) + digits.remove(digits.len()-1) * 10
}

fn get_input(src: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let res = reqwest::blocking::Client::new().get(src)
        .header(USER_AGENT, "")
        .header(COOKIE, "session=53616c7465645f5f64963b27dc1a96e3b498e57c182fdeeaffe976345070fc3d0f22c88c1d91459933897ad530ab45683a294585bdea24b9a034f97ee4c8d46b")
        .send()?;

    let res_body= res.text()?;

    let res_body = res_body.split('\n').collect::<Vec<_>>();

    let mut input = Vec::<String>::new();
    for value in res_body {
        if !value.is_empty() {
            input.push(value.to_string());
        }
    }

    Ok(input)

}
