use std::cmp;

use advent_of_code_2023::tokenizer::{Token, tokenize};

#[derive(Debug, Clone)]
struct Game {
    index: u32,
    matches: Vec<(u32, u32, u32)>,
}
impl Game {
    fn new() -> Game { 
        Game{ index: 0, matches: vec!((0,0,0)) } 
    }
}

#[allow(dead_code)]
pub fn get_answer_part_1(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>>{
    let mut sum = 0;
    for Game{ index, matches } in get_result(input.join("\n"))? {
        if matches.iter().all(|&(reds, greens, blues)|  reds <= 12 && greens <= 13 && blues <= 14) { 
            sum += index;
        }
    }

    Ok(sum)
}

pub fn get_answer(input: Vec<String>) -> Result<u32, Box<dyn std::error::Error>> {
    //TODO: give input as full buffer
    let sum = get_result(input.join("\n"))?.iter()
        .fold(0, |sum, Game{ matches, .. }| {
            let (reds, greens, blues) = matches.iter()
                .fold((0,0,0), |(min_reds, min_greens, min_blues), &(reds, greens, blues)| {
                    (cmp::max(min_reds, reds), cmp::max(min_greens, greens), cmp::max(min_blues, blues))
                });
            sum + reds * greens * blues
        });

    Ok(sum)
}

fn get_result(input: String) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    //println!("{input:?}");
    let tokens = tokenize(input.as_bytes())?;
    //println!("{tokens:#?}");
    let instructions = parse_tokens(tokens)?;
    //println!("{instructions:?}");
    let result = evaluate_stack(instructions)?;
    //println!("{result:?}");

    Ok(result)
}

fn parse_tokens(input: Vec<Token>) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    use advent_of_code_2023::tokenizer::ParseTokenError;

    let mut tokens = Vec::<&Token>::new();
    let instructions = input.iter().map(|token| { 
        tokens.push(token);
        let instructions = match &tokens[..] {
            [Token::Keyword("Game") , Token::Count(num), Token::Seperator(":")] => vec!(Instruction::Index(*num)),
            [Token::Count(num), Token::Identifier(color)] => match color.as_str() 
            {
                "red"   => vec!(Instruction::Red(*num)),
                "green" => vec!(Instruction::Green(*num)),
                "blue"  => vec!(Instruction::Blue(*num)),
                _ => return Err(ParseTokenError::InvalidToken(color.to_string())),
            },
            [Token::Seperator(",")] => vec!(),
            [Token::Seperator(";")] => vec!(Instruction::Round),
            //TODO add rule for end of line sepeerator
            [Token::Seperator("\n") | Token::EndOfInput] => vec!(Instruction::Game),
            _ => return Ok(vec!()),
        };

        tokens.clear(); 

        Ok(instructions)
    })
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .flatten()
    .collect();

    Ok(instructions)
}

#[derive(Debug)]
enum Instruction { Index(u32), Red(u32), Green(u32), Blue(u32), Round, Game}
fn evaluate_stack(stack: Vec<Instruction>) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    //println!("{stack:?}");
    //Evaluate stack
    let mut curr_game = Game::new();
    let results = stack.iter().flat_map(|instruction| match instruction { 
        Instruction::Index(num) => { 
            curr_game.index = *num; 
            vec!()
        }, 
        Instruction::Red(num) => { 
            curr_game.matches.last_mut().unwrap().0 = *num;
            vec!()
        },
        Instruction::Green(num) => { 
            curr_game.matches.last_mut().unwrap().1 = *num;
            vec!()
        }, 
        Instruction::Blue(num) => { 
            curr_game.matches.last_mut().unwrap().2 = *num; 
            vec!()
        }, 
        Instruction::Round => { 
            curr_game.matches.push((0,0,0));
            vec!()
        }
        Instruction::Game => {
            let result = curr_game.clone();
            curr_game = Game::new();
            vec!(result)
        }
    }).collect();

    Ok(results)
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
