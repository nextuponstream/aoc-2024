pub struct WordSearch {
    width: usize,
    height: usize,
    lines: Vec<String>,
}

impl WordSearch {
    pub fn new(input: impl Into<String>) -> Self {
        let input = input.into();
        let mut lines = input.lines();

        let width = lines.next().unwrap().len();
        for line in lines.into_iter() {
            assert_eq!(width, line.len());
        }

        Self {
            lines: input.lines().map(|s| s.to_string()).collect(),
            width,
            height: input.lines().count(),
        }
    }

    pub fn occurences(&self, word: &str) -> usize {
        let first_character: char = word.chars().nth(0).unwrap();

        let mut sum = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.lines[y].chars().nth(x).unwrap() == first_character {
                    sum = sum + self.count_occurences_at(x, y, word);
                }
            }
        }

        sum
    }

    fn count_occurences_at(&self, x: usize, y: usize, word: &str) -> usize {
        let mut count = 0;
        let can_look_right= x + word.len() <= self.width;
        let can_look_left = x >= word.len() - 1;
        let can_look_up = y >= word.len() - 1;
        let can_look_down = y + word.len() <= self.height;
        if can_look_right {
            // look right
            if &self.lines[y][x..x + word.len()] == word {
                count = count + 1;
            }
        }
        if can_look_left {
            let maybe = &self.lines[y][x + 1 - word.len()..x + 1];
            let maybe: String = maybe.chars().rev().collect();
            if &maybe == word {
                count = count + 1;
            }
        }
        if can_look_down {
            let mut to_test = String::new();
            for i in y..y+word.len() {
                to_test.push_str(&self.lines[i][x..x+1]);
            }

            if &to_test == word {
                count = count + 1;
            }
        }
        if can_look_up {
            let mut to_test = String::new();
            let l = y+1-word.len();
            let char_range = l..=y;
            for i in char_range.rev() {
                to_test.push_str(&self.lines[i][x..x+1]);
            }

            if &to_test == word {
                count = count + 1;
            }
        }

        if can_look_up && can_look_right {
            let mut to_test = String::new();
            for i in 0..word.len() {
                to_test.push_str(&self.lines[y - i][x+i..x+i+1])
            }

            if &to_test == word {
                count = count + 1;
            }
        }
        if can_look_down && can_look_right {
            let mut to_test = String::new();
            for i in 0..word.len() {
                to_test.push_str(&self.lines[y + i][x+i..x+i+1])
            }

            if &to_test == word {
                count = count + 1;
            }
        }
        if can_look_up && can_look_left {
            let mut to_test = String::new();
            for i in 0..word.len() {
                to_test.push_str(&self.lines[y - i][x-i..x-i+1])
            }

            if &to_test == word {
                count = count + 1;
            }
        }
        if can_look_down && can_look_left {
            let mut to_test = String::new();
            for i in 0..word.len() {
                to_test.push_str(&self.lines[y + i][x-i..x-i+1])
            }

            if &to_test == word {
                count = count + 1;
            }
        }

        count
    }

    pub fn count_xmas(&self) -> usize {
        let mut sum = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.lines[y].chars().nth(x).unwrap() == 'A' {
                    sum = sum + self.is_christmas(x, y);
                }
            }
        }

        sum
    }

    pub fn is_christmas(&self, x: usize, y: usize) -> usize {
        let can_look_right= x <= self.width - 2;
        let can_look_left = x >= 1;
        let can_look_up = y >= 1;
        let can_look_down = y <= self.height - 2;

        if can_look_right && can_look_up && can_look_left && can_look_down {
            let diag_1_1 = &self.lines[y - 1][x-1..x];
            let diag_1_2 = &self.lines[y + 1][x+1..x+2];
            let diag_1 = format!("{diag_1_1}A{diag_1_2}");

            let diag_1_1 = &self.lines[y - 1][x+1..x+2];
            let diag_1_2 = &self.lines[y + 1][x-1..x];
            let diag_2 = format!("{diag_1_1}A{diag_1_2}");

            match (diag_1.as_str(), diag_2.as_str()) {
                ("MAS", "MAS") => 1,
                ("SAM", "MAS") => 1,
                ("MAS", "SAM") => 1,
                ("SAM", "SAM") => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_occurences_at_right() {
        let ws = WordSearch::new("XMAS");
        assert_eq!(1, ws.count_occurences_at(0, 0, "XMAS"));
        assert_eq!(1, ws.occurences("XMAS"));
    }

    #[test]
    fn count_occurences_at_left() {
        let ws = WordSearch::new("SAMX");
        assert_eq!(1, ws.count_occurences_at(3, 0, "XMAS"));
    }

    #[test]
    fn count_occurences_at_down() {
        let ws = WordSearch::new("X\nM\nA\nS");
        assert_eq!(1, ws.count_occurences_at(0, 0, "XMAS"));
    }

    #[test]
    fn count_occurences_at_up() {
        let ws = WordSearch::new("S\nA\nM\nX");
        assert_eq!(1, ws.count_occurences_at(0, 3, "XMAS"));
    }
}
