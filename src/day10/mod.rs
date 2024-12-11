use std::collections::HashSet;

pub fn part1() {
    let input = include_str!("../../res/day10/part1");
    println!("{}", trail_scores(input));
}

struct Map {
    topology: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

fn trail_scores(input: &str) -> u32 {
    let topology = input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = Map::new(topology);
    map.find_trail_scores().into_iter().sum()
}

impl Map {
    fn new(topology: Vec<Vec<u8>>) -> Map {
        let width = topology[0].len();
        let height = topology.len();
        Map { topology, width, height }
    }

    fn find_trail_scores(&self) -> Vec<u32> {
        self.topology
            .iter()
            .enumerate()
            .flat_map(|(row, row_vec)| row_vec
                .iter()
                .enumerate()
                .filter(|(col, height)| **height == 0)
                .map(move |(col, _)| self.scores_for_trail_head((row, col)))
            )
            .collect::<Vec<_>>()
    }

    fn scores_for_trail_head(&self, trail_head: (usize, usize)) -> u32 {
        self.progress_trail(trail_head.0, trail_head.1, 0).len() as u32
    }

    fn progress_trail(&self, row: usize, col: usize, target_height: u8) -> HashSet<(usize, usize)> {
        let mut result_set = HashSet::new();
        if self.topology[row][col] != target_height {
            return result_set;
        }
        if target_height == 9 {
            result_set.insert((row, col));
            return result_set;
        }
        if row > 0 {
             result_set.extend(self.progress_trail(row-1, col, target_height + 1).into_iter());
        }
        if col > 0 {
            result_set.extend(self.progress_trail(row, col-1, target_height + 1).into_iter());
        }
        if row < self.height - 1 {
            result_set.extend(self.progress_trail(row + 1, col, target_height + 1).into_iter());
        }
        if col < self.width - 1 {
            result_set.extend(self.progress_trail(row, col + 1, target_height + 1).into_iter());
        }
        result_set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(36, trail_scores(&input));
    }
}