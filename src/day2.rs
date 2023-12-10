use advent_of_code_2023::tokenizer::Token;

pub fn get_answer(input: Vec<String>) -> u32{
    for line in input {
        parse_input(&line);
    }

    return 0;
}

fn parse_input(input: &String) -> (usize, Vec<(u32, u32, u32)>) {
    //Sperate games into matches
    let mut matches: Vec<&str> = input.split(';').collect();
    let first_str = matches.first_mut().unwrap();
    let parts: Vec<&str> = first_str.split(':').collect();

    //Extract game index
    let mut index = parts[0].to_string();
    index.retain(|c| c.is_numeric()); 
    let index = index.parse::<u32>().unwrap();
    *first_str = parts[1];

    println!("Round {index}:\n");
    for round in matches {
        get_result(round);
    }

    (0, vec!((0, 0, 0)))
}

fn get_result(input: &str) -> Option<(u32, u32, u32)> {
    let mut token_stack = Vec::<Token>::new();
    for mut word in input.split_whitespace() {
        word = word.trim();

        token_stack.push(word.parse::<Token>().ok()?);
    }
   

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let test_values = vec!(
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", [(4,0,3),(1,2,6),(0,2,0)], true)
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", [()], true)
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", [()], false)
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", [()], false)
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", [()], true)
        );

        for (input, output) in test_values{
            //assert_eq!((&input.to_string()), output) 
        }
    }
}
