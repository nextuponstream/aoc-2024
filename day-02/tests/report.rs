use day_02::Report;

#[test]
fn test_safety_of_report_example_1() {
    let r = Report(vec![7, 6, 4, 2, 1, ]);
    assert!(r.safe());
}

#[test]
fn test_safety_of_report_example_2() {
    let r = Report(vec![1, 2, 7, 8, 9]);
    assert!(!r.safe());
}
#[test]
fn test_safety_of_report_example_3() {
    let r = Report(vec![9, 7, 6, 2, 1]);
    assert!(!r.safe());
}
#[test]
fn test_safety_of_report_example_4() {
    let r = Report(vec![1, 3, 2, 4, 5]);
    assert!(!r.safe());
}
#[test]
fn test_safety_of_report_example_5() {
    let r = Report(vec![8, 6, 4, 4, 1]);
    assert!(!r.safe());
}
#[test]
fn test_safety_of_report_example_6() {
    let r = Report(vec![1, 3, 6, 7, 9]);
    assert!(r.safe());
}

#[test]
fn parse_report() {
    let input = "1 2 3 4\n";
    let r: Report = input.try_into().unwrap();
    assert_eq!(r.0, vec![1,2,3,4]);
}