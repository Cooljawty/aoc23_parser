use reqwest::header::{ USER_AGENT, COOKIE, CONNECTION };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = "53616c7465645f5f64963b27dc1a96e3b498e57c182fdeeaffe976345070fc3d0f22c88c1d91459933897ad530ab45683a294585bdea24b9a034f97ee4c8d46b";
    let puzzle_input_src = "https://adventofcode.com/2023/day/1/input";

    let puzzle_input = get_input(puzzle_input_src, auth)?;

    let answer = puzzle_input.iter().map(|value| parse_input(value));
    println!("+__\n{}", answer.sum::<u32>());
    Ok(())
}

//Parses line and returns the first and last digit as two digit number
fn parse_input(value: &String) -> u32 {
    let mut value = value.trim().to_lowercase();

    let mut digits = Vec::<u32>::new();
    while !value.is_empty() { match pop_number(&mut value) {
        Some(num) => { digits.push(num as u32); }, //Should work as long as usize is atleast 4bits
        None => { break; }
    }}

    if digits.len() == 1 { digits.push(digits[0]) };

    print!(" {}\n", digits[digits.len()-1] + digits[0] * 10);

    digits.remove(digits.len()-1) + digits.remove(0) * 10
}

//Parses line returning first digit and the remaining line
fn pop_number(value: &mut String) -> Option<usize> {
    static TOKENS: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 

    let mut substr = String::new();
    while !value.is_empty() {
        if !value.is_empty() {substr.push(value.remove(0)); }

        //Try getting numerics first
        let digits: Vec<&str> = substr.matches(char::is_numeric).collect();
        println!("{:?}", digits);
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

fn get_input(src: &str, auth: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let res = reqwest::blocking::Client::new().get(src)
        .header(USER_AGENT, "")
        .header(COOKIE, format!("session={}", auth))
        .header(CONNECTION, "keep-alive")
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day1_part2() {
        let test_values = vec!(
            ("onthreethreeboat", 33),
            ("twoeightwo", 28),
            ("bxfour3two2sb4twondmfdpsz", 42),
        );

        for (input, output) in test_values{
            assert_eq!(parse_input(&input.to_string()), output, 
            "Given '{input}', expected {output} but got {}", parse_input(&input.to_string()))
        }
    }
}
