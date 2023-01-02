use day_2::*;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(input);
    println!(" {:?}", result);
}
