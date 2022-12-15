use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part1(input.to_string()), 157);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(file);
    println!("{:?}", result);
}

fn part1(file: String) -> u32 {
    let result = file
    .lines()
    .map(|line| {
        let middle = line.len()/2;
        let end = line.len();

        let first = &line[0..middle].chars().collect::<HashSet<char>>();
        let second = &line[middle..end].chars().collect::<HashSet<char>>();
        
        let repeated_element: char = first
                                    .intersection(&second) // Returns &char elements in both sets
                                    .map(|c| *c) // &char -> char
                                    .collect::<Vec<char>>() // Collect as Vec<char>
                                    [0]; // Assume there is only one

        match repeated_element {
            'a' ..= 'z' => {
                repeated_element as u32 - 96 // Shift ASCII lowercase range 97..122 to range 0..26
            },
            'A' ..= 'Z' => {
                repeated_element as u32 - 64 + 26 // Shift ASCII uppercase range 65..90 to range 0..26 and finally to range 27..52
            },
            _ => panic!("Unexpected char")
        }

    })
    .sum::<u32>();

    result
}

