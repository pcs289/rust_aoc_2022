#[cfg(test)]
mod day_1 {
    use super::*;

        const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 24000);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 45000);
    }
    

}

pub fn part1(file: String) -> i32 {
    let mut summed_calories: Vec<i32> = file
    .split("\n\n")
    .map(|line| {
        line
        .split("\n")
        .map(|e| e.parse::<i32>().unwrap())
        .sum::<i32>()
    }).collect();

    summed_calories.sort_by(|a, b| b.cmp(a));

    summed_calories[0]
}

pub fn part2(file: String) -> i32 {
    let mut summed_calories: Vec<i32> = file
    .split("\n\n")
    .map(|line| {
        line
        .split("\n")
        .map(|e| e.parse::<i32>().unwrap())
        .sum::<i32>()
    }).collect();

    summed_calories.sort_by(|a, b| b.cmp(a));

    summed_calories
    .into_iter()
    .take(3)
    .sum::<i32>()
}
