#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = r#""#;
        assert_eq!(part2(input.to_string()), 0);
    }

}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let result = part2(file);
    println!("{:?}", result);
}

fn part2(file: String) -> i32 {
    0
}
