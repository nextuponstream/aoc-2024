pub fn parse_input(input: String) -> usize {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut left_list: Vec<usize> = vec![];
    let mut right_list: Vec<usize> = vec![];
    for line in lines {
        let (left, right) = parse_into_left_and_right(line);
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let mut sum: usize = 0;

    for i in 0..left_list.len() {
        sum = sum
            + match (left_list[i], right_list[i]) {
                (l, r) if l <= r => r - l,
                (l, r) if r < l => l - r,
                _ => unreachable!(),
            };
    }

    sum
}

fn parse_into_left_and_right(line: &str) -> (usize, usize) {
    let collected: Vec<&str> = line.splitn(2, " ").collect();
    if collected.len() < 2 {
        return (0, 0);
    }
    let left = collected[0];
    let right = collected[1];
    let left = left.parse().unwrap();
    let right = right.trim().parse().unwrap();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_example() {
        assert_eq!(parse_into_left_and_right("3\t4"), (3, 4))
    }
}
