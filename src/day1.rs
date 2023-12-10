use std::collections::HashMap;

pub fn get_answer(input: Vec<String> ) -> u32 {
    input.iter()
        .map(|value| parse_input(value))
        .sum::<u32>()
}
//Parses line and returns the first and last digit as two digit number
fn parse_input(value: &String) -> u32 {
    let mut value = value.trim().to_lowercase();

    let mut digits = pop_number(&mut value);

    if digits.len() == 1 { digits.push(digits[0]) };

    digits.remove(digits.len()-1) + digits.remove(0) * 10
}

//Parses line returning first digit and the remaining line
fn pop_number(value: &mut String) -> Vec<u32> {
    let tokens: HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
    ]); 

    let token_matches: Vec<(usize, u32)> = tokens.iter()
        .flat_map(|(t, _)| value.match_indices(t).collect::<Vec<(usize, &str)>>())
        .collect::<Vec<(usize,&str)>>()
        .iter()
        .map(|(i, t)| (*i, tokens[t]))
        .collect::<Vec<(usize, u32)>>();

    let digit_matches: Vec<(usize, u32)> = value.match_indices(char::is_numeric)
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .map(|(i, d)| (*i, d.parse::<u32>().unwrap()))
        .collect::<Vec<(usize, u32)>>();

    let mut search: Vec<(usize, u32)> = digit_matches.into_iter().chain(token_matches.into_iter()).collect();

    search.sort_by(|a, b| a.0.cmp(&b.0) );
    search.into_iter().map(|(_, v)| v).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
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
