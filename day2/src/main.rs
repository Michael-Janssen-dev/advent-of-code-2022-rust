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
    fn from_str_p1(x: &str) -> RPS{
        if x == "A" || x == "X" {
            return RPS::Rock{}
        }
        if x == "B" || x == "Y" {
            return RPS::Paper{}
        }
        return RPS::Scissors{}
    }
    fn from_i32(x: i32) -> RPS {
        match x {
            0 => RPS::Rock{},
            1 => RPS::Scissors{},
            _ => RPS::Paper{}
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            RPS::Rock{} => 0,
            RPS::Scissors{} => 1,
            RPS::Paper{} => 2
        }
    }

    fn from_str_p2(other: &RPS, x: &str) -> RPS{
        if x == "X" {
            return RPS::from_i32((other.to_i32() + 1).rem_euclid(3))
        }
        if x == "Y" {
            return RPS::from_i32(other.to_i32())
        }
        return RPS::from_i32((other.to_i32() - 1).rem_euclid(3))
    }

    fn outcome(&self, other: &RPS) -> u32 {
        let mut outcome = 0;
        match self {
            RPS::Rock{} => {
                outcome += 1;
                if let RPS::Scissors{} = other {
                    outcome += 6
                }
            },
            RPS::Paper{} => {
                outcome += 2;
                if let RPS::Rock{} = other {
                    outcome += 6
                }
            },
            RPS::Scissors{} => {
                outcome += 3;
                if let RPS::Paper{} = other {
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
        .map(|x| x.splitn(2, " "))
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
        .map(|x| x.splitn(2, " "))
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
