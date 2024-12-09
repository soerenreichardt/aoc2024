use std::collections::{HashMap, HashSet};

pub fn part1() {
    let input = include_str!("../../res/day8/part1");
    println!("{}", anti_node_count(input));
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

fn anti_node_count(input: &str) -> u32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let anti_node_positions = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (row, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(col, c)| if c != '.' {
                    map
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(Position { x: col as i64, y: row as i64})
                });
            map
        })
        .into_iter()
        .map(|(_, positions)| anti_nodes_for_frequency(positions, width, height))
        .reduce(|mut lhs, rhs| {
            lhs.extend(rhs);
            lhs
        })
        .unwrap();

    anti_node_positions.iter().for_each(|p| println!("{:?}", p));
    
    anti_node_positions.len() as u32
}

fn anti_nodes_for_frequency(positions: Vec<Position>, width: usize, height: usize) -> HashSet<Position> {
    let mut anti_nodes = HashSet::new();
    for (start, &pos1) in positions.iter().enumerate() {
        for &pos2 in positions[start..].iter() {
            if pos1 == pos2 {
                continue;
            }
            let diff = pos2 - pos1;
            let a1 = pos1 - diff;
            let a2 = pos2 + diff;

            if check_bounds(a1, width, height) {
                anti_nodes.insert(a1);
            }

            if check_bounds(a2, width, height) {
                anti_nodes.insert(a2);
            }
        }
    }

    anti_nodes
}

fn check_bounds(pos: Position, width: usize, height: usize) -> bool {
    if pos.x >= 0 && (pos.x as usize) < width && pos.y >= 0 && (pos.y as usize) < height {
        return true;
    }
    false
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        assert_eq!(14, anti_node_count(&input));
    }
}
