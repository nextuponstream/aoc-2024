use day_01::parse_input;

fn main() {
    let input = std::fs::read_to_string("./day-01/input").unwrap();

    let output = parse_input(input);

    println!("{output}");
}
