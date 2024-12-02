pub struct Report(pub Vec<usize>);

impl Report {
    pub fn safe(&self) -> bool {
        let first = self.0[0];

        let second = self.0[1];
        let increasing = match (first, second) {
            (f, s) if f == s => {
                return false;
            }
            (f, s) if f < s => true,
            (f, s) if f > s => false,
            _ => unreachable!(),
        };

        let mut copy = self.0.clone();
        copy.sort();
        if !increasing {
            copy.reverse();
        }
        if self.0 != copy {
            return false;
        }

        let min_delta = 1;
        let max_delta = 3;
        if increasing {
            let mut old = first;
            for number in &self.0[1..] {
                let delta = number - old;
                if !(min_delta <= delta && delta <= max_delta) {
                    return false;
                }
                old = number.clone();
            }
        } else {
            let mut old = first;
            for number in &self.0[1..] {
                let delta = old - number;
                if !(min_delta <= delta && delta <= max_delta) {
                    return false;
                }
                old = number.clone();
            }
        }

        true
    }
}

impl From<&str> for Report {
    fn from(input: &str) -> Self {
        let input = input.replace("\n", "");
        let raw_numbers: Vec<&str> = input.split(' ').collect();
        let mut numbers = vec![];
        for raw_number in raw_numbers {
            let number: usize = raw_number.parse().unwrap();
            numbers.push(number);
        }

        Report(numbers)
    }
}
