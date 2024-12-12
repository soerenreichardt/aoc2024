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
    let stones = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut memory: HashMap<(u8, usize), Vec<usize>> = HashMap::new();
    simulate_blink(stones, 0, num_blinks, &mut memory).len()
}

fn simulate_blink(stones: Vec<usize>, step: u8, limit: u8, memory: &mut HashMap<(u8, usize), Vec<usize>>) -> Vec<usize> {
    if step == limit {
        return stones;
    }
    stones.into_iter().flat_map(|s| match memory.get(&(step, s)) {
        Some(result) => result.clone(),
        None => simulate_one_stone(s, step, limit, memory)
    }).collect::<Vec<_>>()
}

fn simulate_one_stone(stone: usize, step: u8, limit: u8, memory: &mut HashMap<(u8, usize), Vec<usize>>) -> Vec<usize> {
    let iteration = apply_rules(stone);
    let result = simulate_blink(iteration, step + 1, limit, memory);
    memory.insert((step, stone), result.clone());
    result
}

fn apply_rules(stone: usize) -> Vec<usize> {
    match stone {
        0 => vec![1],
        d => {
            let s = d.to_string();
            let len = s.len();
            if len % 2 == 0 {
                return split_number(s, len)
            }
            vec![d * 2024]
        },
    }
}

fn split_number(s: String, len: usize) -> Vec<usize> {
    let left = s[0..len/2].parse::<usize>().unwrap();
    let right = s[len/2..len].parse::<usize>().unwrap();
    vec![left, right]
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