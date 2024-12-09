type Rule = (usize, usize);
type Update = Vec<usize>;
pub struct Manual {
    pub rules: Vec<Rule>,
    pub updates: Vec<Update>,
}

impl Manual {
    pub fn new(input: impl Into<String>) -> Self {
        let input = input.into();
        let (rules_input, update_input) = input.split_once("\n\n").unwrap();
        let rules = rules_input
            .split("\n")
            .map(|rule| {
                let (l, r) = rule.split_once("|").unwrap();
                let left: usize = l.parse().unwrap();
                let right: usize = r.parse().unwrap();
                (left, right)
            })
            .collect();
        let updates = update_input
            .split("\n")
            .map(|update| {
                let v: Vec<usize> = update
                    .split(",")
                    .map(|v| {
                        let value: usize = v.parse().unwrap();
                        value
                    })
                    .collect();
                v
            })
            .collect();

        Self { rules, updates }
    }

    pub fn sum_of_valid_updates(&self) -> usize {
        let mut sum: usize = 0;
        for update in &self.updates {
            if self.is_valid(&update) {
                let middle = update.len() / 2;
                let to_add = update[middle];
                sum = sum + to_add;
            }
        }

        sum
    }

    fn is_valid(&self, update: &Update) -> bool {
        let mut update_is_valid = true;
        for rule in &self.rules {
            let left_index = update.iter().position(|&r| r == rule.0);
            let right_index = update.iter().position(|&r| r == rule.1);

            if let (Some(left_pos), Some(right_pos)) = (left_index, right_index) {
                if left_pos > right_pos {
                    update_is_valid = false;
                }
            }
        }

        update_is_valid
    }

    pub fn sum_of_invalid_updates_after_reordering(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if !self.is_valid(&update) {
                sum = sum + self.reorder_and_give_middle(&update);
            }
        }

        sum
    }

    /// Not efficient but correct nonetheless
    fn reorder_and_give_middle(&self, update: &Update) -> usize {
        let mut update = update.clone();
        for _rule in &self.rules {
            for rule in &self.rules {
                let left_index = update.iter().position(|&r| r == rule.0);
                let right_index = update.iter().position(|&r| r == rule.1);
                if let (Some(left_pos), Some(right_pos)) = (left_index, right_index) {
                    if left_pos > right_pos {
                        update.swap(left_pos, right_pos)
                    }
                }
            }
        }

        let middle = update.len() / 2;
        update[middle]
    }
}
