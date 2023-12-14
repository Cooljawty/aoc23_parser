use std::cmp;

use advent_of_code_2023::tokenizer::{Token, tokenize};

#[allow(dead_code)]
pub fn get_answer_part_1(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>>{
    let mut sum = 0;
    'line: for line in input {
        let mut index: u32 = 0;
        for result in get_result(line, &mut index)? {
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
        for result in get_result(line, &mut index)? {
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

fn get_result(input: String, index: &mut u32) -> Result<Vec<(u32, u32, u32)>, Box<dyn std::error::Error>> {
    let tokens = tokenize(input.as_bytes())?;
    let symbols = parse_tokens(tokens);
    let result = evaluate_stack(symbols, index);

    Ok(result)
}

fn parse_tokens(input: Vec<Token>) -> Vec<Symbol> {
    let mut tokens = Vec::<&Token>::new();
    input.iter().flat_map(|token| { 
        tokens.push(token);
        let symbols = match &tokens[..] {
            [Token::Count(num), Token::Identifier(color)] => match color.as_str() {
                "red" => vec!(Symbol::Red(*num)),
                "green" => vec!(Symbol::Green(*num)),
                "blue" => vec!(Symbol::Blue(*num)),
                _ => {panic!("Loose color matching. {color} is not a valid color!")}
            },
            [Token::Keyword("Game") , Token::Count(num), Token::Seperator(":")] => vec!(Symbol::Index(*num), Symbol::Match),
            [Token::Seperator(";") | Token::EndOfInput] => { vec!(Symbol::Match) },
            [Token::Seperator(",")] => { Vec::<Symbol>::new() },//continue; },
            _ => { return Vec::<Symbol>::new() },//continue; },
        };

        println!("tokens: {tokens:?} => symbols: {symbols:?}");
        tokens.clear(); 

        symbols
    }).collect()
}

#[derive(Debug)]
enum Symbol { Index(u32), Red(u32), Green(u32), Blue(u32), Match}
fn evaluate_stack(stack: Vec<Symbol>, index: &mut u32) -> Vec<(u32, u32, u32)> {
    //Evaluate stack
    let mut curr_match = (0,0,0);
    stack.iter().flat_map(|symbol| match symbol { 
        Symbol::Index(num) => { 
            *index = *num; 
            Vec::<(u32,u32,u32)>::new()
        }, 
        Symbol::Red(num) => { curr_match.0 = *num;
            Vec::<(u32,u32,u32)>::new()
        },
        Symbol::Green(num) => { 
            curr_match.1 = *num;
            Vec::<(u32,u32,u32)>::new()
        }, 
        Symbol::Blue(num) => { 
            curr_match.2 = *num; 
            Vec::<(u32,u32,u32)>::new()
        }, 
        Symbol::Match => { 
            let tmp = curr_match;
            curr_match = (0,0,0); 
            vec!(tmp)
        }
    }).collect()
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
