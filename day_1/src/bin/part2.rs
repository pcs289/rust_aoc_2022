#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(part2(input.to_string()), 45000);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let result = part2(file);
    println!("{:?}", result);
}

fn part2(file: String) -> i32 {
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
