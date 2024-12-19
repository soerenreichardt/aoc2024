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

struct Component {
    name: char,
    positions: HashSet<Position>,
    min: Position,
    max: Position,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
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
                let uncontained_neighbors = neighborhood.into_iter().filter(|p| !self.positions.contains(&p)).count() as u32;
                perimeter + uncontained_neighbors
            })
    }

    fn edge_positions(&self) -> HashSet<Position> {
        self.positions.iter()
            .filter_map(|p| if p.neighborhood().into_iter().all(|p| self.positions.contains(&p)) {
                None
            } else {
                Some(p.clone())
            })
            .collect::<HashSet<_>>()
    }

    fn edge_segments(&self) -> u32 {
        let mut segments = 0;

        for x in self.min.x..=self.max.x {
            let mut last_left_contained = true;
            let mut last_right_contained = true;
            for y in self.min.y..=self.max.y {
                let position: Position = (x, y).into();
                if !self.positions.contains(&position) {
                    last_left_contained = true;
                    last_right_contained = true;
                    continue;
                }
                let [left, _, right, _] = position.neighborhood();
                let left_contained = self.positions.contains(&left);
                if left_contained {
                } else {
                }
                if !left_contained && last_left_contained {
                    segments += 1;
                }
                last_left_contained = left_contained;

                let right_contained = self.positions.contains(&right);
                if right_contained {
                } else {
                }
                if !right_contained && last_right_contained {
                    segments += 1;
                }
                last_right_contained = right_contained;
            }
        }

        for y in self.min.y..=self.max.y {
            let mut last_top_contained = true;
            let mut last_bottom_contained = true;
            for x in self.min.x..=self.max.x {
                let position: Position = (x, y).into();
                if !self.positions.contains(&position) {
                    last_top_contained = true;
                    last_bottom_contained = true;
                    continue;
                }

                let [_, top, _, bottom] = position.neighborhood();
                let top_contained = self.positions.contains(&top);
                if top_contained {
                } else {
                }
                if !top_contained && last_top_contained {
                    segments += 1;
                }
                last_top_contained = top_contained;

                let bottom_contained = self.positions.contains(&bottom);
                if bottom_contained {
                } else {
                }
                if !bottom_contained && last_bottom_contained {
                    segments += 1;
                }
                last_bottom_contained = bottom_contained;
            }
        }

        segments
    }
}

impl Position {
    fn neighborhood(&self) -> [Position;4] {
        let x = self.x;
        let y = self.y;
        [
            Position { x: x-1, y      },
            Position { x,      y: y-1 },
            Position { x: x+1, y      },
            Position { x,      y: y+1 },
        ]
    }
}

fn fence_price(input: &str) -> u32 {
    components(input).into_iter().fold(0, |acc, component| {
        let area = component.area();
        let perimeter = component.perimeter();
        acc + area * perimeter
    })
}

fn fence_price_discount(input: &str) -> u32 {
    components(input).into_iter().map(|component| {
        let segments = component.edge_segments();
        let area = component.area();
        segments * area
    }).sum()
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
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    stack.push((x, y));
    while let Some((x, y)) = stack.pop() {
        if board[y][x] != name {
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        positions.insert((x as i64, y as i64).into());
        min_x = min_x.min(x as i64);
        max_x = max_x.max(x as i64);
        min_y = min_y.min(y as i64);
        max_y = max_y.max(y as i64);

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

    let min = (min_x, min_y).into();
    let max = (max_x, max_y).into();
    Component { name, positions, min, max }
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
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!(80, fence_price_discount(input));
    }
}