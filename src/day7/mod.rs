use winnow::Parser;

pub fn part1() {
    let input = include_str!("../../res/day7/part1");
    println!("{}", compute_calibrations(input));
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul
}

impl Operation {
    fn evaluate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}

impl From<usize> for Operation {
    fn from(value: usize) -> Self {
        match value {
            0 => Operation::Add,
            1 => Operation::Mul,
            _ => unreachable!()
        }
    }
}

fn compute_calibrations(input: &str) -> usize {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(result, operands)| (result.parse::<usize>().unwrap(), operands.split_whitespace().map(|operand| operand.parse::<usize>().unwrap()).collect::<Vec<_>>()))
        .map(|tuple| solve_equation(tuple))
        .sum()
}

fn solve_equation((expected_result, operands): (usize, Vec<usize>)) -> usize {
    let num_options = (1usize << operands.len() - 1);

    for permutation in 0..num_options {
        let mut equation_result = 0usize;
        for (index, operand) in operands.iter().enumerate() {
            if index == 0 {
                equation_result = *operand;
                continue;
            }
            let operation_code = (permutation >> (index - 1)) & 1usize;
            equation_result = Operation::from(operation_code).evaluate(equation_result, *operand);
       }

        if equation_result == expected_result {
            return equation_result;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(3749, compute_calibrations(input));
    }
}