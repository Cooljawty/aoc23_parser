use reqwest::header::{ USER_AGENT, COOKIE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input_src = "https://adventofcode.com/2023/day/1/input";

    let puzzle_input = get_input(puzzle_input_src)?;

    let answer = puzzle_input.iter().map(|value| parse_input(value));
    print!("{}", answer.fold(0, |a,n| a + n));
    Ok(())
}

fn parse_input(value: &String) -> u32 {
    let mut value = value.clone();
    value.retain(|c| c.is_numeric());
    value.replace_range(1..(if value.len() > 1  {value.len()-1} else {1}), "");

    let mut result = value.clone();
    if value.len() == 1 { result.insert_str(0, value.as_mut_str()) };
    
    result.parse::<u32>().unwrap()
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
