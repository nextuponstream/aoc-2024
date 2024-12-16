/// x, y and direction
type Position = (usize, usize, Direction);

#[derive(Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
pub struct Map {
    obstacles: Vec<Vec<bool>>,
    starting_position: Position,
}

impl Map {
    pub fn new(input: impl Into<String>) -> Self {
        let mut obstacles = vec![];
        let mut starting_position = None;
        for (y, line) in input.into().lines().enumerate() {
            let mut sub = vec![];
            for (x, point) in line.chars().enumerate() {
                sub.push(point == '#');

                if point == '^' {
                    if starting_position.is_some() {
                        unreachable!()
                    }
                    starting_position = Some((x, y, Direction::UP));
                }
            }
            obstacles.push(sub)
        }

        Self {
            obstacles,
            starting_position: starting_position.unwrap(),
        }
    }

    pub fn visited(&self) -> usize {
        let mut position = self.starting_position;
        let mut visited: Vec<Vec<bool>> = self
            .obstacles
            .iter()
            .map(|l| l.iter().map(|_| false).collect())
            .collect();
        visited[self.starting_position.1][self.starting_position.0] = true;

        while !self.exited(position) {
            // self.print_position(&position, &visited);
            let (new_position, moved) = self.next_move(position);
            position = new_position;
            if moved {
                visited[position.1][position.0] = true;
            }
        }
        // self.print_position(&position, &visited);
        Self::visited_count(&visited)
    }

    fn exited(&self, position: Position) -> bool {
        match position.2 {
            Direction::UP => position.1 == 0,
            Direction::DOWN => position.1 == self.obstacles.len() - 1,
            Direction::LEFT => position.0 == 0,
            Direction::RIGHT => position.0 == self.obstacles[0].len() - 1,
        }
    }

    /// next position and true if could move in direction
    fn next_move(&self, position: Position) -> (Position, bool) {
        let y = position.1;
        let x = position.0;
        match position.2 {
            Direction::UP => {
                let blocked = self.obstacles[y - 1][x];
                if blocked {
                    ((x, y, Direction::RIGHT), false)
                } else {
                    ((position.0, y - 1, position.2), true)
                }
            }
            Direction::DOWN => {
                let blocked = self.obstacles[y + 1][x];
                if blocked {
                    ((x, y, Direction::LEFT), false)
                } else {
                    ((position.0, y + 1, position.2), true)
                }
            }
            Direction::LEFT => {
                let blocked = self.obstacles[y][x - 1];
                if blocked {
                    ((x, y, Direction::UP), false)
                } else {
                    ((x - 1, position.1, position.2), true)
                }
            }
            Direction::RIGHT => {
                let blocked = self.obstacles[y][x + 1];
                if blocked {
                    ((x, y, Direction::DOWN), false)
                } else {
                    ((x + 1, position.1, position.2), true)
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_position(&self, position: &Position, visited: &Vec<Vec<bool>>) {
        let mut map: Vec<Vec<char>> = self
            .obstacles
            .iter()
            .map(|line| {
                let l = line
                    .iter()
                    .map(|obstacle| if *obstacle { 'x' } else { '.' })
                    .collect();
                l
            })
            .collect();

        match position.2 {
            Direction::UP => {
                map[position.1][position.0] = '^';
            }
            Direction::DOWN => {
                map[position.1][position.0] = 'V';
            }
            Direction::LEFT => {
                map[position.1][position.0] = '<';
            }
            Direction::RIGHT => {
                map[position.1][position.0] = '>';
            }
        }

        let visited: usize = Self::visited_count(&visited);
        println!("visited {visited}");
        for line in map {
            let l: String = line.iter().collect();
            println!("{l}")
        }
    }

    fn visited_count(visited: &Vec<Vec<bool>>) -> usize {
        visited
            .iter()
            .map(|v| v.iter().map(|s| if *s { 1 } else { 0 }).sum::<usize>())
            .sum()
    }
}
