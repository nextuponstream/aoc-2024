use day_05::Manual;

#[test]
fn parse_example() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let manual = Manual::new(input);

    let rule: (usize, usize) = (47, 53);
    assert!(manual.rules.contains(&rule));
    assert!(manual.updates.contains(&vec![75, 29, 13]));
    assert_eq!(143, manual.sum_of_valid_updates())
}
#[test]
fn sum_of_invalid_example_1() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let manual = Manual::new(input);

    let rule: (usize, usize) = (47, 53);
    assert!(manual.rules.contains(&rule));
    assert!(manual.updates.contains(&vec![75, 29, 13]));
    assert_eq!(123, manual.sum_of_invalid_updates_after_reordering())
}
