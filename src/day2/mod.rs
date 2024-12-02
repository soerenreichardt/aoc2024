mod red_nodes_reports;

pub fn part1() {
    let input = std::str::from_utf8(include_bytes!("../../res/day2/part1")).unwrap();;
    println!("{}", red_nodes_reports::safe_reports(input, false));
}

pub fn part2() {
    let input = std::str::from_utf8(include_bytes!("../../res/day2/part1")).unwrap();;
    println!("{}", red_nodes_reports::safe_reports(input, true));
}