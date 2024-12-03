mod mull_it_over;

pub fn part1() {
    let mut input = std::str::from_utf8(include_bytes!("../../res/day3/part1")).unwrap();
    println!("{}", mull_it_over::compute_multiplications(&mut input));
}

// pub fn part2() {
//     let input = std::str::from_utf8(include_bytes!("../../res/day1/part1")).unwrap();;
//     println!("{}", crate::day3::historian_hysteria::similarity_score(input));
// }