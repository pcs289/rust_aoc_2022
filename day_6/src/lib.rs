use std::collections::HashSet;

#[cfg(test)]
mod day_6 {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_A: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_B: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_C: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_D: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";


    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 7);
        assert_eq!(part1(INPUT_A.to_string()), 5);
        assert_eq!(part1(INPUT_B.to_string()), 6);
        assert_eq!(part1(INPUT_C.to_string()), 10);
        assert_eq!(part1(INPUT_D.to_string()), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 19);
        assert_eq!(part2(INPUT_A.to_string()), 23);
        assert_eq!(part2(INPUT_B.to_string()), 23);
        assert_eq!(part2(INPUT_C.to_string()), 29);
        assert_eq!(part2(INPUT_D.to_string()), 26);
    }

}

pub fn part1(file: String) -> usize {

    let v: Vec<char> = file.chars().clone().collect();
    let result: usize = file
    .chars()
    .enumerate()
    .skip(4)
    .find_map(|(index, c)| {
        let mut h: HashSet<char> = HashSet::new();
        for i in 1..=4 {
            h.insert(v[index - i]);
        }
        if h.len() >= 4 {
            Some(index)
        } else {
            None
        }
    })
    .unwrap();

    println!("Final Result: {result}");
    result
}

pub fn part2(file: String) -> usize {
    
    let v: Vec<char> = file.chars().clone().collect();
    let result: usize = file
    .chars()
    .enumerate()
    .skip(14)
    .find_map(|(index, c)| {
        let mut h: HashSet<char> = HashSet::new();
        for i in 1..=14 {
            h.insert(v[index - i]);
        }
        if h.len() >= 14 {
            Some(index)
        } else {
            None
        }
    })
    .unwrap();

    println!("Final Result: {result}");
    result
}
