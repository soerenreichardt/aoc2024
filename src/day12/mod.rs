use std::collections::HashSet;
use std::usize;

pub fn part1() {
    let input = include_str!("../../res/day12/part1");
    println!("{}", fence_price(input));
}

pub fn part2() {
    let input = include_str!("../../res/day12/part1");
    println!("{}", fence_price_discount(input));
}

fn fence_price(input: &str) -> u32 {
    components(input).into_iter().fold(0, |acc, component| {
        let area = component.area();
        let perimeter = component.perimeter();
        println!("{} - area: {}, perimeter: {}", component.name, area, perimeter);
        acc + area * perimeter
    })
}

struct Component {
    name: char,
    positions: HashSet<Position>
}

#[derive(PartialEq, Hash, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position { x, y }
    }
}

impl Component {
    fn area(&self) -> u32 {
        self.positions.len() as u32
    }

    fn perimeter(&self) -> u32 {
        self.positions.iter()
            .fold(0, |perimeter, position| {
                let neighborhood = position.neighborhood();
                let out_of_bounds_neighbors = 4 - neighborhood.len() as u32;
                let uncontained_neighbors = neighborhood.into_iter().filter(|p| !self.positions.contains(&p)).count() as u32;
                perimeter + uncontained_neighbors + out_of_bounds_neighbors
            })
    }
}

impl Position {
    fn neighborhood(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let x = self.x;
        let y = self.y;
        if x > 0 {
            positions.push((x-1, y).into())
        }
        if y > 0 {
            positions.push((x, y-1).into())
        }
        positions.push((x+1, y).into());
        positions.push((x, y+1).into());
        positions
    }
}

fn fence_price_discount(input: &str) -> u32 {
    todo!()
}

fn components(input: &str) -> Vec<Component> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    let board = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for (y, row) in board.iter().enumerate() {
        for (x, region) in row.iter().enumerate() {
            if !visited.contains(&(x, y)) {
                components.push(component(*region, (x, y), &board, &mut visited));
            }
        }
    }

    components
}

fn component(name: char, (x, y): (usize, usize), board: &[Vec<char>], visited: &mut HashSet<(usize, usize)>) -> Component {
    let mut stack = Vec::new();
    let mut positions = HashSet::new();

    stack.push((x, y));
    while let Some((x, y)) = stack.pop() {
        if board[y][x] != name {
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        positions.insert((x, y).into());

        if x > 0 {
            stack.push((x - 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if x < board[0].len() - 1 {
            stack.push((x + 1, y));
        }
        if y < board.len() - 1 {
            stack.push((x, y + 1));
        }
    }

    Component { name, positions }
}

#[cfg(test)]
mod tests {
    use crate::day12::{fence_price, fence_price_discount};

    #[test]
    fn part1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        assert_eq!(772, fence_price(input));
    }

    #[test]
    fn part2() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        assert_eq!(80, fence_price_discount(input));
    }
}