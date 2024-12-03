use winnow::ascii::digit1;
use winnow::combinator::{alt, preceded, separated_pair, terminated};
use winnow::{PResult, Parser};

enum Instruction {
    Multiple(u32, u32),
    Do,
    Dont
}

pub fn compute_multiplications(input: &str) -> u32 {
    let mut input = input;
    let mut tuples = Vec::new();
    let mut enabled = true;
    while input.len() > 0 {
        match parse_instruction(&mut input) {
            Ok(instruction) => match instruction {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Multiple(x, y) if enabled => {
                    tuples.push((x, y));
                }
                _ => ()
            },
            Err(_) => input = &mut &input[1..],
        }
    }

    tuples.into_iter().map(|(lhs, rhs)| lhs * rhs).sum()
}

fn parse_instruction(input: &mut &str) -> PResult<Instruction> {
    alt((
        parse_mul_tuple,
        parse_do,
        parse_dont
    )).parse_next(input)
}

fn parse_mul_tuple(input: &mut &str) -> PResult<Instruction> {
    terminated(
        preceded(
            "mul(", 
            separated_pair(
                digit1, 
                ',', 
                digit1
            )
        ),
        ')'
    )
        .parse_next(input)
        .map(|(lhs, rhs)| Instruction::Multiple(lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap()))
}

fn parse_do(input: &mut &str) -> PResult<Instruction> {
    "do()".parse_next(input).map(|_| Instruction::Do)
}

fn parse_dont(input: &mut &str) -> PResult<Instruction> {
    "don't()".parse_next(input).map(|_| Instruction::Dont)
}

#[cfg(test)]
mod tests {
    use crate::day3::mull_it_over::compute_multiplications;

    #[test]
    fn test() {
        assert_eq!(48, compute_multiplications(&mut "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
    }
}
