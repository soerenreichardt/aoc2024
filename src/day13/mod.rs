use std::error::Error;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{preceded, separated_pair};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::Parser;

pub fn part1() {
    let mut input = include_str!("../../res/day13/part1");
    println!("{}", cheapest_wins(&mut input, false));
}

pub fn part2() {
    let mut input = include_str!("../../res/day13/part1");
    println!("{}", cheapest_wins(&mut input, true));
}


fn cheapest_wins(input: &mut &str, scaled: bool) -> i64 {
    input.split("\n\n")
        .map(|mut group| ClawMachine::from(&mut group).cheapest_win(scaled))
        .sum()
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

struct Line(Position, Position);

struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl std::ops::Mul<i64> for Position {
    type Output = Self;

    fn mul(self, value: i64) -> Self::Output {
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

impl Line {
    fn intersection(&self, other: &Self) -> Option<Position> {
        let a1 = self.1.y - self.0.y;
        let b1 = self.0.x - self.1.x;
        let c1 = a1 * self.0.x + b1 * self.0.y;

        let a2 = other.1.y - other.0.y;
        let b2 = other.0.x - other.1.x;
        let c2 = a2 * other.0.x + b2 * other.0.y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0 {
            return None;
        }

        let intersection_x = (b2 * c1 - b1 * c2) / delta;
        let intersection_y = (a1 * c2 - a2 * c1) / delta;

        Some(Position {
            x: intersection_x,
            y: intersection_y,
        })
    }
}

impl From<&mut &str> for ClawMachine {
    fn from(input: &mut &str) -> Self {
        fn parse_button(input: &mut &str, button_name: &str) -> PResult<Position> {
            preceded(format!("Button {}: ", button_name).as_str(), separated_pair(preceded("X+", digit1), ", ", preceded("Y+", digit1)))
                .parse_next(input)
                .map(|(x, y)| Position { x: x.parse::<i64>().unwrap(), y: y.parse::<i64>().unwrap() })
        }

        fn parse_prize(input: &mut &str) -> PResult<Position> {
            preceded("Prize: ", separated_pair(preceded("X=", digit1), ", ", preceded("Y=", digit1)))
                .parse_next(input)
                .map(|(x, y)| Position { x: x.parse::<i64>().unwrap(), y: y.parse::<i64>().unwrap() })
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
    fn cheapest_win(&self, scaled: bool) -> i64 {
        let prize = if scaled { self.prize.clone() + Position { x: 10000000000000, y: 10000000000000 } } else { self.prize.clone() };

        let line_a = Line(Position { x: 0, y: 0 }, self.button_a.clone());
        let line_b = Line(prize.clone() - self.button_b.clone(), prize.clone());

        line_a.intersection(&line_b)
            .map(|intersection_point| {
                let multiplier_a = intersection_point.x / self.button_a.x;
                let multiplier_b = (prize.x - multiplier_a * self.button_a.x) / self.button_b.x;

                if self.button_a.clone() * multiplier_a +  self.button_b.clone() * multiplier_b != prize {
                    0
                } else {
                    3 * multiplier_a + multiplier_b
                }
            })
            .unwrap_or(0)
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
        assert_eq!(480, cheapest_wins(&mut input, false));
    }

    #[test]
    fn part2() {
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
        assert_eq!(480, cheapest_wins(&mut input, true));
    }
}