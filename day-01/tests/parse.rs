use day_01::parse_input;

#[test]
fn example_input() {
   let input = "\
3 4
4 3
2 5
1 3
3 9
3 3";
    assert_eq!(parse_input(input.into()), 11_usize);
}