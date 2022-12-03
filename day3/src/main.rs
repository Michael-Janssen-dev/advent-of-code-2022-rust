#![feature(iter_array_chunks)]
use std::{fs};
use std::collections::{HashSet};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 157)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 70)
}

fn handle_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

fn priority(c: &char) -> u32 {
    let mut x: u32 = *c as u32;
    if x >= 'a' as u32 {
        x -= 'a' as u32;
        x += 1;
    } else {
        x -= 'A' as u32;
        x += 27;
    }
    x
}

fn part_1(inp: &str) -> u32 {
    inp
        .lines()
        .map(|x| {
            let (a, b) = x.split_at(x.bytes().count() / 2);
            let set_1: HashSet<char> = a.chars().collect();
            let set_2: HashSet<char> = b.chars().collect();
            let dif_set = set_1.intersection(&set_2);
            dif_set.into_iter().next().unwrap().clone()
        })
        .map(|x| {
            priority(&x)
        })
        .sum()
}

fn part_2(inp: &str) -> u32 {
    inp
        .lines()
        .array_chunks()
        .map(|[a, b, c]| {
            let set_1: HashSet<char> = a.chars().collect();
            let set_2: HashSet<char> = b.chars().collect();
            let set_3: HashSet<char> = c.chars().collect();
            let dif_set: HashSet<char> = set_1.intersection(&set_2).into_iter().map(|x| x.clone()).collect();
            let diff_set = dif_set.intersection(&set_3);
            diff_set.into_iter().next().unwrap().clone()
        })
        .map(|x| {
            priority(&x)
        })
        .sum()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
