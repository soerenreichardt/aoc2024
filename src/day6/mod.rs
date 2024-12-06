use std::collections::HashSet;
use std::str::FromStr;

pub fn part1() {
    let input = include_str!("../../res/day6/part1");
    println!("{}", guard_path(input));
}

pub fn part2() {
    let input = include_str!("../../res/day6/part1");
    println!("{}", guard_path(input));
}

fn guard_path(input: &str) -> u32 {
    let mut board = Board::from_str(input).unwrap();
    board.predict_guard_movement().0
}

fn obstruction_loops(input: &str) -> u32 {
    let mut board = Board::from_str(input).unwrap();
    board.predict_guard_movement().1
}

#[derive(Debug)]
enum Tile {
    Free,
    Occupied,
    Guard(Direction)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Direction::Left => *self = Direction::Up,
            Direction::Right => *self = Direction::Down,
            Direction::Up => *self = Direction::Right,
            Direction::Down => *self = Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize
}

impl FromStr for Board {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(|c| match c {
                '.' => Tile::Free,
                '#' => Tile::Occupied,
                '^' => Tile::Guard(Direction::Up),
                _ => unreachable!()
            }).collect::<Vec<_>>()).collect::<Vec<_>>();
        let width = tiles[0].len();
        let height = tiles.len();

        Ok(Board { tiles, width , height })
    }
}

impl Board {
    fn predict_guard_movement(&mut self) -> (u32, u32) {
        let mut guard_position = self.find_guard();
        let mut direction = Direction::Up;
        let mut seen_positions = HashSet::new();
        seen_positions.insert(guard_position);
        let mut loops = 0;

        while true {
            match self.move_guard(guard_position, &mut direction) {
                Some(position) => guard_position = position,
                None => break
            }
            if seen_positions.insert(guard_position) {
                loops += 1;
            }
        }

        (seen_positions.len() as u32, loops)
    }

    fn move_guard(&mut self, (x, y): (usize, usize), direction: &mut Direction) -> Option<(usize, usize)> {
        if self.is_outside((x, y), direction) {
            return None;
        }
        let new_guard_position = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        if self.is_occupied(new_guard_position) {
            direction.turn_right();
            return self.move_guard((x, y), direction);
        }
        Some(new_guard_position)
    }

    fn find_guard(&self) -> (usize, usize) {
        for (row, columns) in self.tiles.iter().enumerate() {
            for (column, tile) in columns.iter().enumerate() {
                match tile {
                    Tile::Guard(_) => return (column, row),
                    _ => continue
                }
            }
        }

        panic!("No guard found");
    }

    fn is_outside(&self, (x, y): (usize, usize), direction: &Direction) -> bool {
        match direction {
            Direction::Up => y == 0,
            Direction::Down => y == self.height - 1,
            Direction::Left => x == 0,
            Direction::Right => x == self.width - 1
        }
    }

    fn is_occupied(&self, (x, y): (usize, usize)) -> bool {
        match self.tiles[y][x] {
            Tile::Occupied => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(41, guard_path(input));
    }

    #[test]
    fn part2() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(41, obstruction_loops(input));
    }
}