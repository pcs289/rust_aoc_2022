use std::collections::HashSet;

#[cfg(test)]
mod day_3 {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 157);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 70);
    }

}

pub fn part1(file: String) -> u32 {
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

pub fn part2(file: String) -> u32 {
    let file_read = file.lines()
    .map(|l| l.to_string())
    .collect::<Vec<String>>();

    let result = file_read.chunks(3)
    .map(|group| {

        let mut hashsets = group
        .iter()
        .map(|l| l.chars().collect::<HashSet<char>>());

        let mut s = hashsets.next().unwrap();
        for line in hashsets {
            s = s.intersection(&line).copied().collect();
        }
        
        let repeated_element: char = s.into_iter().collect::<Vec<char>>()[0];

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
