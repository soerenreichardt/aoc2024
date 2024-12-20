use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{preceded, separated_pair};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::Parser;

pub fn part1() {
    let mut input = include_str!("../../res/day13/part1");
    println!("{}", cheapest_wins(&mut input));
}

fn cheapest_wins(input: &mut &str) -> usize {
    input.split("\n\n")
        .map(|mut group| ClawMachine::from(&mut group).cheapest_win())
        .sum()
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl std::ops::Mul<usize> for Position {
    type Output = Self;

    fn mul(self, value: usize) -> Self::Output {
        Position {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<&mut &str> for ClawMachine {
    fn from(input: &mut &str) -> Self {
        fn parse_button(input: &mut &str, button_name: &str) -> PResult<Position> {
            preceded(format!("Button {}: ", button_name).as_str(), separated_pair(preceded("X+", digit1), ", ", preceded("Y+", digit1)))
                .parse_next(input)
                .map(|(x, y)| Position { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() })
        }

        fn parse_prize(input: &mut &str) -> PResult<Position> {
            preceded("Prize: ", separated_pair(preceded("X=", digit1), ", ", preceded("Y=", digit1)))
                .parse_next(input)
                .map(|(x, y)| Position { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() })
        }

        let button_a = parse_button(input, "A").unwrap();
        newline::<&str, ContextError>.parse_next(input).unwrap();
        let button_b = parse_button(input, "B").unwrap();
        newline::<&str, ContextError>.parse_next(input).unwrap();
        let prize = parse_prize(input).unwrap();

        Self { button_a, button_b, prize }
    }
}

impl ClawMachine {
    fn cheapest_win(&self) -> usize {
        let Position { x: prize_x, y: prize_y} = self.prize;

        let min_multiple_a = prize_x.div_ceil(self.button_a.x).min(prize_y.div_ceil(self.button_a.y));

        let mut a_multiple = min_multiple_a;
        let mut b_multiple = 0usize;

        while a_multiple > 0 {
            let claw_position = self.button_a.clone() * a_multiple + self.button_b.clone() * b_multiple;
            if claw_position == self.prize {
                return 3 * a_multiple + b_multiple;
            }
            if claw_position.x > self.prize.x && claw_position.y > self.prize.y {
                a_multiple -= 1;
            } else {
                b_multiple += 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::cheapest_wins;

    #[test]
    fn part1() {
        let mut input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(480, cheapest_wins(&mut input));
    }
}