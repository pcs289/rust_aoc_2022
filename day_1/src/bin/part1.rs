#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
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
        assert_eq!(part1(input.to_string()), 24000);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(file);
    println!("{:?}", result);
}

fn part1(file: String) -> i32 {
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

