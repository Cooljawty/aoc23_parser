use advent_of_code_2023::tokenizer::Token;

pub fn get_answer(input: Vec<String>) -> u32{
    for line in input {
        parse_input(&line);
        break;
    }

    return 0;
}

fn parse_input(input: &String) -> Option<(usize, Vec<(u32, u32, u32)>)> {
    //Sperate games into matches
    let matches: Vec<&str> = input.split(|c: char| {
        c.is_whitespace() || [';', ':', ','].contains(&c)
    })
    .collect();

    //Extract game index
    let Ok(Token::Count(index)) = matches[1].to_string().parse::<Token>() else {panic!("Missing game index!")};

    get_result(matches);

    Some((index.try_into().ok()?, vec!((0, 0, 0))))
}

fn get_result(input: Vec<&str>) -> Option<Vec<(u32, u32, u32)>> {
    //println!("\nline: {:?}", input);
    let mut token_stack = vec!(Token::StartLine);
    for mut word in input {
        word = word.trim();
        match word.parse::<Token>().ok() {
            Some(token) => { token_stack.push(token); },
            None => {},
        }
        //println!("input: {:?}\nstack: {:?}", word, token_stack)
    }

    while let Some(token) = token_stack.pop() { 
        todo!("Evaluate stack")
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
