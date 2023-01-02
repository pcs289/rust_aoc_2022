use std::ops::Range;

#[cfg(test)]
mod day_4 {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 2);
    }


    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 4);
    }

}


pub fn part1(file: String) -> i32 {
    let result = file
    .lines()
    .map(|line| {

        let ranges = line
        .split(",")
        .map(|range| {
            let start: i32 = range.split("-").nth(0).unwrap().parse::<i32>().unwrap();
            let end: i32 = range.split("-").nth(1).unwrap().parse::<i32>().unwrap();
            std::ops::Range { start, end: end + 1  }
        })
        .collect::<Vec<Range<i32>>>();
        
        (ranges[0].clone(), ranges[1].clone())
    })
    .filter(|(a, b)| {
        let first = a.clone();
        let second = b.clone();
        let is_a_in_b = first.into_iter().all(|c| b.contains(&c));
        let is_b_in_a = second.into_iter().all(|c| a.contains(&c));
        is_a_in_b || is_b_in_a
    })
    .count();
    

    result as i32
}


pub fn part2(file: String) -> i32 {
    let result = file
    .lines()
    .map(|line| {

        let ranges = line
        .split(",")
        .map(|range| {
            let start: i32 = range.split("-").nth(0).unwrap().parse::<i32>().unwrap();
            let end: i32 = range.split("-").nth(1).unwrap().parse::<i32>().unwrap();
            std::ops::Range { start, end: end + 1  }
        })
        .collect::<Vec<Range<i32>>>();
        
        (ranges[0].clone(), ranges[1].clone())
    })
    .filter(|(a, b)| {
        let first = a.clone();
        let second = b.clone();
        let is_any_a_in_b = first.into_iter().any(|c| b.contains(&c));
        let is_any_b_in_a = second.into_iter().any(|c| a.contains(&c));
        is_any_a_in_b || is_any_b_in_a
    })
    .count();
    

    result as i32
}
