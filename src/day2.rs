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

#[derive(Debug, PartialEq, Eq)]
enum Color { Red, Green, Blue }
fn get_result(mut input: Vec<Token>, index: &mut u32) -> Vec<(u32, u32, u32)> {
    //Evaluate stack
    let mut count: u32 = 0;
    let mut curr_color: Option<Color> = None;

    let mut matches = Vec::<(u32, u32, u32)>::new();
    let mut curr_match = (0,0,0);
    while let Some(token) = input.pop() { 
        match token {
            Token::Keyword(color) => {
                if color != "Game" {
                    let new_color = match color.as_str() {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => {panic!("Loose color matching.\nThis should not happen!")}
                    };

                    match curr_color {
                        //Reset on repead color token
                        Some(ref color) if *color == new_color  => {count = 0}
                        Some(ref color) => { 
                            match color {
                                Color::Red => { curr_match.0 = count; },
                                Color::Green => { curr_match.1 = count; },
                                Color::Blue => { curr_match.2 = count; },
                            }
                        },
                        None => match new_color {
                                Color::Red => { curr_match.0 = count; },
                                Color::Green => { curr_match.1 = count; },
                                Color::Blue => { curr_match.2 = count; },
                        },
                    };

                    curr_color = Some(new_color);
                } else if color == "Game" {
                    *index = count;
                }
            },
            Token::Count(digit) => count = digit, 
            Token::Seperator(s) => match s.as_str() {
                ";"|":" => { 
                    match curr_color {
                        Some(ref color) => { 
                            match color {
                                Color::Red => { curr_match.0 = count; },
                                Color::Green => { curr_match.1 = count; },
                                Color::Blue => { curr_match.2 = count; },
                            }
                        },
                        None => {},
                    };
                    matches.push(curr_match); 
                    curr_match = (0,0,0); 
                    count = 0;
                }
                _ => {}
            }
            _ => {}, 
        }
    }
    matches.reverse();
    
    matches

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let test_input = vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        let result = get_answer_part_1(test_input.iter().map(|s| s.to_string()).collect());
        assert_eq!(result, 8, "Expected {:?}, got {:?}", 8, result); 
    }

    #[test]
    fn part2() {
        let test_input = vec!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        let result = get_answer(test_input.iter().map(|s| s.to_string()).collect());
        assert_eq!(result, 2286, "Expected {:?}, got {:?}", 2286, result); 
    }
}
