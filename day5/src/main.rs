use std::{fs};

use regex::Regex;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("input/test.txt")), "CMZ")
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("input/test.txt")), "MCD")
}

fn init_stack(s: &str) -> Vec<Vec<char>> {
    let n = (s.lines().next().unwrap().len() + 1 )/ 4;
    let mut result: Vec<Vec<char>> = Vec::with_capacity(n);
    (0..n).for_each(|_|result.push(Vec::new()));
    s.lines().rev().skip(1).for_each(|x| (0..n).for_each(|i| {
        if !x.chars().nth(1 + 4*i).unwrap().is_whitespace() {
            result.get_mut(i).unwrap().push(x.chars().nth(1 + 4*i).unwrap())
        }
    }));

    result
}

type HandleInput = (Vec<Vec<char>>, String);

fn handle_input(file: &str) -> HandleInput {
    let file = fs::read_to_string(file).unwrap();
    let (init, moves) = file
        .split_once("\n\n")
        .unwrap();
    let stacks: Vec<Vec<char>> = init_stack(init);
    (stacks, moves.to_string())
}


fn part_1((mut stacks, moves): HandleInput) -> String {
    let regex = Regex::new(r"move (?P<x>\d+) from (?P<a>\d+) to (?P<b>\d+)").unwrap();
    moves.lines().for_each(|line| {
        let captures = regex.captures(line).unwrap();
        let x: usize = captures.name("x").unwrap().as_str().parse().unwrap();
        let a: usize = captures.name("a").unwrap().as_str().parse().unwrap();
        let b: usize = captures.name("b").unwrap().as_str().parse().unwrap();
        (0..x).for_each(|_| {
            let p = stacks.get_mut(a - 1).unwrap().pop().unwrap();
            stacks.get_mut(b - 1).unwrap().push(p);
        })
    });
    stacks.iter_mut().map(|x: &mut Vec<char>|x.pop().unwrap()).collect()
}

fn part_2((mut stacks, moves): HandleInput) -> String {
    let regex = Regex::new(r"move (?P<x>\d+) from (?P<a>\d+) to (?P<b>\d+)").unwrap();
    moves.lines().for_each(|line| {
        let captures = regex.captures(line).unwrap();
        let x: usize = captures.name("x").unwrap().as_str().parse().unwrap();
        let a: usize = captures.name("a").unwrap().as_str().parse().unwrap();
        let b: usize = captures.name("b").unwrap().as_str().parse().unwrap();
        let mut items: Vec<char> = Vec::new();

        (0..x).for_each(|_| {
            items.push(stacks.get_mut(a - 1).unwrap().pop().unwrap());
        });
        items.iter().rev().for_each(|p| {
            stacks.get_mut(b - 1).unwrap().push(*p);
        });
    });
    stacks.iter_mut().map(|x: &mut Vec<char>|x.pop().unwrap()).collect()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(inp.clone()));
    println!("Part 2: {}", part_2(inp.clone()));
}
