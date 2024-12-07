use regex::Regex;

pub struct CorruptedMemory(pub String);

impl CorruptedMemory {
    pub fn scan_multiplication(&self) -> Result<usize, ()> {
        let re = Regex::new(r#"mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\)"#).unwrap();

        let multiplications: Vec<usize> = re.captures_iter(self.0.as_str()).map(|c| {
            let left = c.name("left").unwrap().as_str();
            let left: usize = left.parse().unwrap();
            let right = c.name("right").unwrap().as_str();
            let right: usize = right.parse().unwrap();
            left * right
        }).collect();

        if multiplications.is_empty() {
            Err(())
        } else {
            Ok(multiplications.iter().sum())
        }
    }
}