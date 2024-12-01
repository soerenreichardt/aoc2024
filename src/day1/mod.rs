mod historian_hysteria;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../res/day1/part1")).unwrap();;
    println!("{}", historian_hysteria::pair_distances(input));
}