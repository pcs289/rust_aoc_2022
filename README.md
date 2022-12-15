# 🎄🦀 Rust - Advent Of Code 2022

This repository contains my particular implementation of the daily challenges for [Advent of Code 2022](https://adventofcode.com). This is done as a learning exercise to dive deeper into [Rust programming language](https://rust-lang.org/learn).

It is organized as a Rust workspace with multiple Rust crates named `day_X` (a number between `1` and `25`)* each representing a day of December in the advent calendar 🎄🦀. Each daily challenge contains at least 2 parts included at `day_X/src/bin` folder and an input file at `day_X/src/input.txt`.

## Run `day_X` binaries
It contains two parts.
```shell
cd day_X
cargo test
cargo run --bin part1
cargo run --bin part2
```

*`day_n` works as a template Cargo project
