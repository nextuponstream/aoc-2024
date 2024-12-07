use day_04::WordSearch;

fn main() {
    let input = std::fs::read_to_string("./day-04/input").unwrap();

    let ws = WordSearch::new(input);

    println!("{}", ws.occurences("XMAS"))
}
