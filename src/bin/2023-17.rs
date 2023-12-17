use pathfinding::directed::dijkstra::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Up, Direction::Down],
            Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    straight: u8,
}

impl State {
    fn successors(&self, map: &Vec<Vec<u32>>) -> Vec<(State, u32)> {
        let mut successors = Vec::new();
        let mut directions = self.direction.turn();
        if self.straight < 3 {
            directions.push(self.direction);
        }
        for direction in directions {
            let (dx, dy) = direction.offset();
            let (x, y) = (self.x as isize + dx, self.y as isize + dy);
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if x >= map.len() || y >= map[x].len() {
                continue;
            }
            successors.push((
                State {
                    x,
                    y,
                    direction,
                    straight: if direction == self.direction {
                        self.straight + 1
                    } else {
                        1
                    },
                },
                map[x][y],
            ));
        }
        successors
    }

    fn success(&self, map: &Vec<Vec<u32>>) -> bool {
        self.x == map.len() - 1 && self.y == map[0].len() - 1
    }
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(map: &Vec<Vec<u32>>) -> u32 {
    let start = State {
        x: 0,
        y: 0,
        direction: Direction::Down,
        straight: 0,
    };
    let goal = |state: &State| state.success(map);
    let successors = |state: &State| state.successors(map);
    dijkstra(&start, successors, goal).unwrap().1
}

fn main() {
    let map = parse(include_str!("../input/2023-17.txt"));

    println!("Part one: {}", part_one(&map));
    // println!("Part two: {}", part_two(&map));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE)), 102);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(&parse(EXAMPLE)), 64);
    // }
}
