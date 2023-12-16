use reqwest::header::{ USER_AGENT, COOKIE, CONNECTION };

mod day1; 
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = "53616c7465645f5f64963b27dc1a96e3b498e57c182fdeeaffe976345070fc3d0f22c88c1d91459933897ad530ab45683a294585bdea24b9a034f97ee4c8d46b";
    let puzzle_input_src = vec!(
        "https://adventofcode.com/2023/day/1/input",
        "https://adventofcode.com/2023/day/2/input",
    );

    let mut puzzle_input = puzzle_input_src.into_iter().filter_map(|src| get_input(src, auth).ok());
    
    //Solve Day 1 part 2
    match puzzle_input.next() {
        Some(src) => { println!("Day 1: {}", day1::get_answer(src.split('\n').map(|s| s.to_string()).collect::<Vec<_>>())) },
        None => { return Err("Could not get Day 1 input".into()) },
    }

    //Solve Day 2 part 1
    match puzzle_input.next() {
        Some(src) => { println!("Day 2: {}", day2::get_answer(src)?) },
        None => { return Err("Could not get Day 2 input".into()) },
    }

    Ok(())
}

fn get_input(src: &str, auth: &str) -> Result<String, Box<dyn std::error::Error>>{
    let res = reqwest::blocking::Client::new().get(src)
        .header(USER_AGENT, "")
        .header(COOKIE, format!("session={}", auth))
        .header(CONNECTION, "keep-alive")
        .send()?;

    let res_body= res.text()?;

    Ok(res_body.to_string())

}

