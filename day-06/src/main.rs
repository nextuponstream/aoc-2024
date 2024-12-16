use day_06::Map;

fn main() {
    let input = std::fs::read_to_string("./day-06/input").unwrap();
    let map = Map::new(input);
    println!("{}", map.visited())
}
