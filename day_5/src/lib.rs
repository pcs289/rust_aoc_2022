use nom::{IResult, sequence::{delimited, preceded}, bytes::complete::tag, multi::{separated_list1, many1}, branch::alt};
use nom::character::complete::{newline, space1, digit1, multispace1, self, alpha1};

#[cfg(test)]
mod day_5 {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), "CMZ".to_string());
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), "MCD".to_string());
    }

}

#[derive(Debug)]
struct Move {
    src: u32,
    dst: u32,
    amount: u32
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, src) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, dst) = complete::u32(input)?;
    Ok((
        input,
        Move {
            amount,
            src: src - 1,
            dst: dst - 1,
        },
    ))
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(
            complete::char('['),
            alpha1,
            complete::char(']'),
        ),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) =
        separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn parse_file(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) =
        separated_list1(newline, parse_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) =
        many1(preceded(space1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) =
        separated_list1(newline, parse_move)(input)?;

    let mut crates_vertical: Vec<Vec<Option<&str>>> =
        vec![];
    for _ in 0..=crates_horizontal.len() {
        crates_vertical.push(vec![]);
    }
    for vec in crates_horizontal.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            crates_vertical[i].push(c.clone())
        }
    }
    let final_crates: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (final_crates, moves)))
}

pub fn part1(file: String) -> String {
    let (_, (mut crate_stacks, moves)) = parse_file(&file).unwrap();

    for Move { amount, src, dst } in moves.iter() {
        let len = crate_stacks[*src as usize].len();
        let drained = crate_stacks[*src as usize]
            .drain((len - *amount as usize)..)
            .rev()
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[*dst as usize].push(c);
        }
    }

    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}

pub fn part2(file: String) -> String {
    let (_, (mut crate_stacks, moves)) = parse_file(&file).unwrap();

    for Move { amount, src, dst } in moves.iter() {
        let len = crate_stacks[*src as usize].len();
        let drained = crate_stacks[*src as usize]
            .drain((len - *amount as usize)..)
            .collect::<Vec<&str>>();
        for c in drained.iter() {
            crate_stacks[*dst as usize].push(c);
        }
    }

    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}
