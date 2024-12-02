
pub fn safe_reports(input: &str, with_tolerance: bool) -> u32 {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .filter(|report| {
            if with_tolerance {
                check_report_with_tolerance(report)
            } else  {
                check_report(report)
            }
        })
        .count() as u32
}

fn check_report(report: &[i32]) -> bool {
    let descending = report[0] - report[1] > 0;
    report.windows(2)
        .map(|pair| pair[0] - pair[1])
        .all(|diff| if descending {
            diff > 0 && diff >= 1 && diff <= 3
        } else {
            diff < 0 && diff <= -1 && diff >= -3
        })
}

fn check_report_with_tolerance(report: &Vec<i32>) -> bool {
    if check_report(&report) {
        return true;
    }
    
    for i in 0..report.len() {
        let mut cloned_report = report.clone();
        cloned_report.remove(i);
        if check_report(&cloned_report) {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use crate::day2::red_nodes_reports::safe_reports;

    #[test]
    fn test_report() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!(2, safe_reports(input, false));
        assert_eq!(4, safe_reports(input, true));
    }
}