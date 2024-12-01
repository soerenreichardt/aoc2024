use std::collections::HashMap;

pub fn pair_distances(input: &str) -> u32 {
    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|parts| (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap()))
        .fold((Vec::new(), Vec::new()), |(mut left_vec, mut right_vec), (lhs, rhs)| {
            left_vec.push(lhs);
            right_vec.push(rhs);
            
            (left_vec, right_vec)
        });
    
    left.sort();
    right.sort();
    
    left.iter().zip(right.iter()).map(|(lhs, rhs)| lhs.abs_diff(*rhs)).sum()
} 

pub fn similarity_score(input: &str) -> u32 {
    let (left, right) = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|parts| (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap()))
        .fold((Vec::new(), HashMap::new()), |(mut left_vec, mut right_vec), (lhs, rhs)| {
            left_vec.push(lhs);
            *right_vec.entry(rhs).or_insert(0u32) += 1;

            (left_vec, right_vec)
        });
    
    left.iter().map(|num| right.get(num).unwrap_or(&0) * num).sum()
}