use day_04::WordSearch;

#[test]
fn example_1() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let ws = WordSearch::new(input.to_string());
    assert_eq!(18, ws.occurences("XMAS"))
}
#[test]
fn example_2() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let ws = WordSearch::new(input.to_string());
    assert_eq!(9, ws.count_xmas())
}

#[test]
fn sub_example_1() {
    let ws = WordSearch::new("SAMX");
    assert_eq!(1, ws.occurences("XMAS"))
}

#[test]
fn sub_example_2() {
    let ws = WordSearch::new("XMAS");
    assert_eq!(1, ws.occurences("XMAS"))
}

#[test]
fn sub_example_3() {
    let ws = WordSearch::new("SAMXMAS");
    assert_eq!(2, ws.occurences("XMAS"))
}
