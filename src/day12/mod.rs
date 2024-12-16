use std::collections::{HashMap, HashSet};

pub fn part1() {
    let input = include_str!("../../res/day12/part1");
    println!("{}", fence_price(input));
}

fn fence_price(input: &str) -> u32 {
    let mut visited = HashSet::new();
    let mut fence_prices = HashMap::new();
    let board = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for (y, row) in board.iter().enumerate() {
        for (x, region) in row.iter().enumerate() {
            if !visited.contains(&(x, y)) {
                let component = component(*region, (x, y), &board, &mut visited);
                fence_prices.entry(region).or_insert(Vec::new()).push(component);
            }
        }
    }

    fence_prices.values().flat_map(|components| components.iter()).fold(0, |acc, (perimeter, area)| acc + area * perimeter)
}

fn component(region: char, (x, y): (usize, usize), board: &[Vec<char>], visited: &mut HashSet<(usize, usize)>) -> (u32, u32) {
    let mut stack = Vec::new();

    let mut perimeter = 0;
    let mut area = 0;

    stack.push((x, y));
    while let Some((x, y)) = stack.pop() {
        if board[y][x] != region {
            perimeter += 1;
            continue;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        area += 1;
        visited.insert((x, y));

        if x > 0 {
            stack.push((x - 1, y));
        } else {
            perimeter += 1;
        }
        if y > 0 {
            stack.push((x, y - 1));
        } else {
            perimeter += 1;
        }
        if x < board[0].len() - 1 {
            stack.push((x + 1, y));
        } else {
            perimeter += 1;
        }
        if y < board.len() - 1 {
            stack.push((x, y + 1));
        } else {
            perimeter += 1;
        }
    }

    (perimeter as u32, area as u32)
}

#[cfg(test)]
mod tests {
    use crate::day12::fence_price;

    #[test]
    fn part1() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        assert_eq!(772, fence_price(input));
    }
}