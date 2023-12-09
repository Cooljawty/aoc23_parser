use std::collections::HashMap;
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
    println!("value: {:?}", value);

    let mut digits = pop_number(&mut value);
    println!("digits: {:?}", digits);

    if digits.len() == 1 { digits.push(digits[0]) };

    print!(" {}\n", digits[digits.len()-1] + digits[0] * 10);

    digits.remove(digits.len()-1) + digits.remove(0) * 10
}

//Parses line returning first digit and the remaining line
fn pop_number(value: &mut String) -> Vec<u32> {
    let tokens: HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
    ]); 

    let token_matches: Vec<(usize, u32)> = tokens.iter()
        .flat_map(|(t, _)| value.match_indices(t).collect::<Vec<(usize, &str)>>())
        .collect::<Vec<(usize,&str)>>()
        .iter()
        .map(|(i, t)| (*i, tokens[t]))
        .collect::<Vec<(usize, u32)>>();

    let digit_matches: Vec<(usize, u32)> = value.match_indices(char::is_numeric)
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .map(|(i, d)| (*i, d.parse::<u32>().unwrap()))
        .collect::<Vec<(usize, u32)>>();

    let mut search: Vec<(usize, u32)> = digit_matches.into_iter().chain(token_matches.into_iter()).collect();

    search.sort_by(|a, b| a.0.cmp(&b.0) );
    search.into_iter().map(|(_, v)| v).collect::<Vec<u32>>()
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
    use super::*;

    #[test]
    fn day1_part2() {
        let test_values = vec!(
            ("onthreethreeboat", 33),
            ("twoeightwo", 22),
            ("bxfour3two2sb4twondmfdpsz", 42),
            ("sevenine", 79),
        );

        for (input, output) in test_values{
            assert_eq!(parse_input(&input.to_string()), output, 
            "Given '{input}', expected {output} but got {}", parse_input(&input.to_string()))
        }
    }
}
