use advent_of_code_2023::tokenizer::Token;

pub fn get_answer(input: Vec<String>) -> u32{
    for line in input {
        println!("{}", line);
        parse_input(&line);
        break;
    }

    return 0;
}

fn parse_input(input: &String) -> Option<Vec<(u32, u32, u32)>> {
    //Sperate games into matches
    let matches: Vec<&str> = input
        .split(|c: char| { c.is_whitespace() })
        //Split seperators to sepreate tokens
        .flat_map(|w| w.split(';')).map(|p| match p {"" => ";", _ => p})
        .flat_map(|w| w.split(':')).map(|p| match p {"" => ":", _ => p})
        .flat_map(|w| w.split(',')).map(|p| match p {"" => ",", _ => p})
        .collect();
    
    //Extract game index
    let Ok(Token::Count(index)) = matches[1].to_string().parse::<Token>() else {panic!("Missing game index!")};

    get_result(matches)

    //Some((index.try_into().ok()?, vec!((0, 0, 0))))
}

#[derive(Debug, PartialEq, Eq)]
enum Color { Red, Green, Blue }
fn get_result(input: Vec<&str>) -> Option<Vec<(u32, u32, u32)>> {
    let mut token_stack = vec!(Token::StartLine);
    for mut word in input {
        word = word.trim();
        match word.parse::<Token>().ok() {
            Some(token) => { token_stack.push(token); },
            None => {},
        }
        //println!("input: {:?}\nstack: {:?}", word, token_stack)
    }

    //Evaluate stack
    let mut count: u32 = 0;
    let mut curr_color: Option<Color> = None;

    let mut matches = Vec::<(u32, u32, u32)>::new();
    let mut curr_match = (0,0,0);
    while let Some(token) = token_stack.pop() { 
        match token {
            Token::Keyword(color) if color != "Game" => {
                let new_color = match color {
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
                //println!("{:?} {:?}", curr_color, curr_match);
            },
            Token::Count(digit) => count = digit, 
            Token::Seperator(s) => match s {
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
    println!("{:?} ", matches);
    matches.reverse();
    
    Some(matches)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let test_values = vec!(
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", vec!((4,0,3),(1,2,6),(0,2,0)), true),
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", vec!((0,2,1),(1,3,4),(0,1,1)), true),
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", vec!((20,8,6),(4,13,5),(1,5,0)), false),
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", vec!((3,1,6),(6,3,0),(14,3,15)), false),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", vec!((6,3,1),(1,2,2)), true),
        );

        for (input, output, _) in test_values{
            assert_eq!(parse_input(&input.to_string()), Some(output)); 
        }
    }
}
