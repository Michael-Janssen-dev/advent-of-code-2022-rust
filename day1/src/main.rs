use std::{fs, num};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 24000)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 45000)
}

fn handle_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

fn part_1(inp: &str) -> u32 {
    inp.split(&"\n\n").into_iter().map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum()).max().unwrap()
}

fn part_2(inp: &str) -> u32 {
    let mut numbers: Vec<u32> = inp.split(&"\n\n").into_iter().map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum()).collect();
    numbers.sort();
    numbers.reverse();
    numbers.into_iter().take(3).sum()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
