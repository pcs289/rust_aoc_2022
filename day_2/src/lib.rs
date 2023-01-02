#[cfg(test)]
mod day_2 {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 15);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 12);
    }

}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

pub fn part1(input: String) -> i32 {

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

#[derive(Clone, Copy, Debug)]
enum Round {
    Draw,
    Lose,
    Win,
}


pub fn part2(input: String) -> i32 {

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
