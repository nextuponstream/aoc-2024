// use day_01::{parse_input};
use day_01::{parse_similarity};

fn main() {
    let input = std::fs::read_to_string("./day-01/input").unwrap();

    // let output = parse_input(input);
    let output = parse_similarity(input);

    println!("{output}");
}
