use std::collections::HashMap;

pub fn part1() {
    let input = include_str!("../../res/day11/part1");
    println!("{}", stone_count(input, 25));
}

pub fn part2() {
    let input = include_str!("../../res/day11/part1");
    println!("{}", stone_count(input, 75));
}

fn stone_count(input: &str, num_blinks: u8) -> usize {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut memory: HashMap<(u8, usize), usize> = HashMap::new();
    simulate_blink(stones, num_blinks, &mut memory)
}

fn simulate_blink(stones: Vec<usize>, limit: u8, memory: &mut HashMap<(u8, usize), usize>) -> usize {
    stones
        .into_iter()
        .map(|s| simulate_one_stone(s, 0, limit, memory))
        .sum()
}

fn simulate_one_stone(stone: usize, step: u8, limit: u8, memory: &mut HashMap<(u8, usize), usize>) -> usize {
    if step == limit {
        return 1;
    }
    match memory.get(&(step, stone)) {
        Some(result) => *result,
        None => {
            let iteration = match stone {
                0 => simulate_one_stone(1, step + 1, limit, memory),
                d => {
                    let s = d.to_string();
                    let len = s.len();
                    if len % 2 == 0 {
                        let (left, right) = split_number(s, len);
                        simulate_one_stone(left, step + 1, limit, memory) +
                            simulate_one_stone(right, step + 1, limit, memory)
                    } else {
                        simulate_one_stone(d * 2024, step + 1, limit, memory)
                    }
                }
            };
            memory.insert((step, stone), iteration);
            iteration
        }
    }
}

fn split_number(s: String, len: usize) -> (usize, usize) {
    let left = s[0..len / 2].parse::<usize>().unwrap();
    let right = s[len / 2..len].parse::<usize>().unwrap();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "125 17";
        assert_eq!(55312, stone_count(input, 25));
    }
}