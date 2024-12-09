use day_05::Manual;

fn main() {
    let input = std::fs::read_to_string("./day-05/input").unwrap();
    let manual = Manual::new(input);
    println!("{}", manual.sum_of_valid_updates())
}
