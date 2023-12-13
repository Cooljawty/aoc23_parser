use std::cmp;

use advent_of_code_2023::tokenizer::{Token, tokenize};

#[allow(dead_code)]
pub fn get_answer_part_1(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>>{
    let mut sum = 0;
    'line: for line in input {
        let mut index: u32 = 0;
        for result in get_result(tokenize(line.as_bytes())?, &mut index) {
            match result {
                (r, g, b) if r > 12 || g > 13 || b > 14 => { 
                    continue 'line;
                },
                _ => {},
            };
        }
        sum += index;
    }

    Ok(sum)
}

pub fn get_answer(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;
    for line in input {
        println!("{line}");
        let mut index: u32 = 0;
        let mut color_count = (0,0,0);
        for result in get_result(tokenize(line.as_bytes())?, &mut index) {
            color_count = (
                cmp::max(result.0, color_count.0), 
                cmp::max(result.1, color_count.1), 
                cmp::max(result.2, color_count.2)
            );
        }
        let power = color_count.0 * color_count.1 * color_count.2; 
        sum += power;
    }

    Ok(sum)
}

fn get_result(mut input: Vec<Token>, index: &mut u32) -> Vec<(u32, u32, u32)> {
    //Evaluate stack
    let mut matches = Vec::<(u32, u32, u32)>::new();
    let mut curr_match = (0,0,0);

    let mut tokens = Vec::<Token>::new();
    while let Some(token) = input.pop() { 
        tokens.push(token);
        println!("{tokens:?}");
        match &tokens[..] {
            [Token::Keyword(color), Token::Count(num)] if color != "Game"=> {
                println!("{color} is {num}");
                match color.as_str() {
                    "red" =>  { curr_match.0 = *num; },
                    "green" => { curr_match.1 = *num; },
                    "blue" => { curr_match.2 = *num; },
                    _ => {panic!("Loose color matching. {color} is not a valid color!")}
                };
                
            },
            [Token::Seperator(s)] => {
                match s.as_str() {
                    ";" => {
                        matches.push(curr_match);
                        println!("result is {curr_match:?}, {matches:?}");
                        curr_match = (0,0,0);
                        tokens.clear();
                    },
                    "," => {
                        //Ignore colon for now
                        tokens.clear();
                    },
                    _ => {continue}
                }
            }
            [ Token::Seperator(s), Token::Count(num), Token::Keyword(k)] if k == "Game" && s == ":" => {
                println!("Game index is {num}");
                matches.push(curr_match);
                curr_match = (0,0,0);
                *index = *num;
            }
            _ => {continue},
        }

        //Clear stack when rule is met
        tokens.clear();
    }

    println!("matches: {matches:?}");
    matches.reverse();
    
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<(), Box<dyn std::error::Error>> {
        let test_input = vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        let result = get_answer_part_1(test_input.iter().map(|s| s.to_string()).collect())?;
        assert_eq!(result, 8, "Expected {:?}, got {:?}", 8, result); 
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Box<dyn std::error::Error>> {
        let test_input = vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        let result = get_answer(test_input.iter().map(|s| s.to_string()).collect())?;
        assert_eq!(result, 2286, "Expected {:?}, got {:?}", 2286, result); 
        Ok(())
    }
}
