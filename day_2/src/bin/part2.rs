#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(part2(input.to_string()), 12);
    }

}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(input);
    println!(" {:?}", result);
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Clone, Copy, Debug)]
enum Round {
    Draw,
    Lose,
    Win,
}

fn part2(input: String) -> i32 {

    let result: i32 = input
    .lines()
    .map(|line| {
        let components = line
        .split(" ")
        .collect::<Vec<&str>>();

        let shape = match components[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Unexpected input value")
        };

        let end = match components[1] {
            "X" => Round::Lose,
            "Y" => Round::Draw,
            "Z" => Round::Win,
            _ => panic!("Unexpected input value")
        };

        (shape, end)
        
    })
    .map(|(other, end)| {

        let end_score = match end {
            Round::Lose => 0,
            Round::Draw => 3,
            Round::Win => 6
        };

        let round_score = match (other, end) {
            (Shape::Rock, Round::Lose) => 3,
            (Shape::Rock, Round::Draw) => 1,
            (Shape::Rock, Round::Win) => 2,
            (Shape::Paper, Round::Lose) => 1,
            (Shape::Paper, Round::Draw) => 2,
            (Shape::Paper, Round::Win) => 3,
            (Shape::Scissors, Round::Lose) => 2,
            (Shape::Scissors, Round::Draw) => 3,
            (Shape::Scissors, Round::Win) => 1,   
        };

        end_score + round_score
    })
    .sum::<i32>();

    result
}
