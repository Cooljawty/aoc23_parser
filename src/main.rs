use reqwest::header::{ USER_AGENT, COOKIE, CONNECTION };

mod day1;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = "53616c7465645f5f64963b27dc1a96e3b498e57c182fdeeaffe976345070fc3d0f22c88c1d91459933897ad530ab45683a294585bdea24b9a034f97ee4c8d46b";
    let puzzle_input_src = "https://adventofcode.com/2023/day/1/input";

    let puzzle_input = get_input(puzzle_input_src, auth)?;

    println!("Day 1: {}", day1::get_answer(puzzle_input));
    Ok(())
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
