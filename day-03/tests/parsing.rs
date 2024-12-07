use day_03::CorruptedMemory;

#[test]
fn valid_parsing_example_1() {
    let input = "mul(44,46)";
    let mem = CorruptedMemory(input.to_string());
    assert_eq!(2024, mem.scan_multiplication().unwrap())
}

#[test]
fn valid_parsing_example_2() {
    let input = "mul(123,4)";
    let mem = CorruptedMemory(input.to_string());
    assert_eq!(492, mem.scan_multiplication().unwrap())
}

#[test]
fn valid_parsing_example_3() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let mem = CorruptedMemory(input.to_string());
    assert_eq!(161, mem.scan_multiplication().unwrap())
}

#[test]
fn parse_do_example_1() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mem = CorruptedMemory(input.to_string());
    assert_eq!(48, mem.scan_do_multiplication().unwrap())
}