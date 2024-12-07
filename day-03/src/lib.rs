use regex::Regex;

pub struct CorruptedMemory(pub String);

impl CorruptedMemory {
    pub fn scan_multiplication(&self) -> Result<usize, ()> {
        let re = Regex::new(r#"mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\)"#).unwrap();

        let multiplications: Vec<usize> = re
            .captures_iter(self.0.as_str())
            .map(|c| {
                let left = c.name("left").unwrap().as_str();
                let left: usize = left.parse().unwrap();
                let right = c.name("right").unwrap().as_str();
                let right: usize = right.parse().unwrap();
                left * right
            })
            .collect();

        if multiplications.is_empty() {
            Err(())
        } else {
            Ok(multiplications.iter().sum())
        }
    }

    pub fn scan_do_multiplication(&self) -> Result<usize, ()> {
        let mut do_string = String::new();
        let mut parse = true;
        for (i, c) in self.0.chars().enumerate() {
            if i < self.0.len() - 4 && &self.0.clone()[i..i + 4] == "do()" {
                parse = true
            } else if i < self.0.len() - 7 && &self.0[i..i + 7] == "don't()" {
                parse = false
            }
            if parse {
                do_string.push(c)
            }
        }

        CorruptedMemory(do_string).scan_multiplication()
    }
}
