use day_7::*;

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let result = part2(file);
    println!("{:?}", result);
}
