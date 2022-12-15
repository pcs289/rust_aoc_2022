use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(part2(input.to_string()), 70);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(file);
    println!("{:?}", result);
}

fn part2(file: String) -> u32 {
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

