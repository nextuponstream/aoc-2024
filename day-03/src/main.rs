use day_03::CorruptedMemory;

fn main() {
    let input = std::fs::read_to_string("./day-03/input").unwrap();
    let mem = CorruptedMemory(input);

    println!("{}", mem.scan_multiplication().unwrap())
}
