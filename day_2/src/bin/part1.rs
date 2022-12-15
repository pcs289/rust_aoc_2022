#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(part1(input.to_string()), 15);
    }

}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = part1(input);
    println!(" {:?}", result);
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

fn part1(input: String) -> i32 {

    let result: i32 = input
    .lines()
    .map(|line| {
        let moves = line
        .split(" ")
        .map(|m| {
            match m {
                "A" | "X" => Shape::Rock,
                "B" | "Y" => Shape::Paper,
                "C" | "Z" => Shape::Scissors,
                _ => panic!("Unexpected input value")
            }
        })
        .collect::<Vec<Shape>>();

        (moves[0], moves[1])
    })
    .map(|(other, me)| {

        let move_score = match me {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let round_score = match (other, me) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,   
        };

        move_score + round_score
    })
    .sum::<i32>();

    result
}
