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