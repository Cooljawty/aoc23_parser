pub fn get_answer(input: Vec<String>) -> u32{
    for line in input { println!("{line}"); }

    return 0;
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
