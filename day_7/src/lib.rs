use std::collections::BTreeMap;

use nom::{IResult, branch::alt, multi::separated_list1, character::complete::{alpha1, newline}, bytes::complete::{tag, is_a}, sequence::separated_pair};

#[cfg(test)]
mod day_n {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT.to_string()), 95437);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT.to_string()), 24933642);
    }

}

#[derive(Debug)]
enum Command<'a> {
    Cd(TraversePath<'a>),
    Ls(Vec<Output<'a>>),
}

#[derive(Debug)]
enum TraversePath<'a> {
    Root,
    Up,
    Down(&'a str),
}
#[derive(Debug)]
enum Output<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn parse_file(input: &str) -> IResult<&str, Output> {
    let (input, (filesize, filename)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm.")
    )(input)?;
    Ok((input, Output::File { size: filesize, name: filename }))
}
fn parse_dir(input: &str) -> IResult<&str, Output> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir) = alpha1(input)?;
    Ok((input, Output::Dir(dir)))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, result) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;
    Ok((input, Command::Ls(result)))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, path) = alt((tag("/"), tag(".."), alpha1))(input)?;
    let command = match path {
        "/" => TraversePath::Root,
        ".." => TraversePath::Up,
        path => TraversePath::Down(path),
    };
    Ok((input, Command::Cd(command)))
}

fn parse_commands(input: &str) -> Vec<Command> {
    let (input, cmd) =
    separated_list1(newline, alt((parse_ls, parse_cd)))(input).expect("Could not parse commands");

    cmd
}

pub fn part1(file: String) -> u32 {
    let commands = parse_commands(&file);

    let (_, sizes) = commands
    .iter()
    .fold(
        (vec![], BTreeMap::new()),
        |(mut context, mut sizes), command| {
            match command {
                Command::Cd(TraversePath::Root) => {
                    context.push("");
                }
                Command::Cd(TraversePath::Up) => {
                    context.pop();
                }
                Command::Cd(TraversePath::Down(name)) => {
                    context.push(name);
                }
                Command::Ls(files) => {
                    let sum = files
                        .iter()
                        .filter_map(|file| {
                            if let Output::File { size, .. } = file {
                                Some(size)
                            } else {
                                None
                            }
                        })
                        .sum::<u32>();
        
                    for i in 0..context.len() {
                        sizes
                            .entry(context[0..=i].to_vec())
                            .and_modify(|v| *v += sum)
                            .or_insert(sum);
                    }
                }
            };
            (context, sizes)
        });

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()

}

pub fn part2(file: String) -> u32 {
    let cmds = parse_commands(&file);
    let (_, sizes) = cmds.iter().fold(
        (vec![], BTreeMap::new()),
        |(mut context, mut sizes), command| {
            match command {
                Command::Cd(TraversePath::Root) => {
                    context.push("");
                }
                Command::Cd(TraversePath::Up) => {
                    context.pop();
                }
                Command::Cd(TraversePath::Down(name)) => {
                    context.push(name);
                }
                Command::Ls(files) => {
                    let sum = files
                        .iter()
                        .filter_map(|file| {
                            if let Output::File { size, .. } = file {
                                Some(size)
                            } else {
                                None
                            }
                        })
                        .sum::<u32>();
        
                    for i in 0..context.len() {
                        sizes
                            .entry(context[0..=i].to_vec())
                            .and_modify(|v| *v += sum)
                            .or_insert(sum);
                    }
                }
            };
            (context, sizes)
    });

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get(&vec![""]).unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least =
        needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string().parse::<u32>().unwrap()
}
