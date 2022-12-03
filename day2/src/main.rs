use std::{fs};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 15)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 12)
}

fn handle_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}
impl RPS {
    fn from_str_p1(x: &str) -> Self{
        if x == "A" || x == "X" {
            return Self::Rock{}
        }
        if x == "B" || x == "Y" {
            return Self::Paper{}
        }
        Self::Scissors{}
    }
    fn from_i32(x: i32) -> Self {
        match x {
            0 => Self::Rock{},
            1 => Self::Scissors{},
            _ => Self::Paper{}
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            Self::Rock{} => 0,
            Self::Scissors{} => 1,
            Self::Paper{} => 2
        }
    }

    fn from_str_p2(other: &Self, x: &str) -> Self{
        if x == "X" {
            return Self::from_i32((other.to_i32() + 1).rem_euclid(3))
        }
        if x == "Y" {
            return Self::from_i32(other.to_i32())
        }
        Self::from_i32((other.to_i32() - 1).rem_euclid(3))
    }

    fn outcome(&self, other: &Self) -> u32 {
        let mut outcome = 0;
        match self {
            Self::Rock{} => {
                outcome += 1;
                if let Self::Scissors{} = other {
                    outcome += 6
                }
            },
            Self::Paper{} => {
                outcome += 2;
                if let Self::Rock{} = other {
                    outcome += 6
                }
            },
            Self::Scissors{} => {
                outcome += 3;
                if let Self::Paper{} = other {
                    outcome += 6
                }
            },
        };
        if self == other {
            outcome += 3
        }
        outcome
    }
}


fn part_1(inp: &str) -> u32 {
    inp
        .lines()
        .map(|x| x.splitn(2, ' '))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(x, y)| {
            let (x, y) = (RPS::from_str_p1(x), RPS::from_str_p1(y));
            y.outcome(&x)
        })
        .sum()
}

fn part_2(inp: &str) -> u32 {
    inp
        .lines()
        .map(|x| x.splitn(2, ' '))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(x, y)| {
            let x = RPS::from_str_p1(x);
            let y = RPS::from_str_p2(&x, y);
            y.outcome(&x)
        })
        .sum()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
