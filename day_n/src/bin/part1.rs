#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#""#;
        assert_eq!(part1(input.to_string()), 0);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(file);
    println!("{:?}", result);
}

fn part1(file: String) -> i32 {
    0
}

