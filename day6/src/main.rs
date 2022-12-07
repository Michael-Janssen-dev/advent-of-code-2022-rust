use std::{fs, collections::HashSet};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 7)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 1)
}

fn handle_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}


fn part_1(inp: &str) -> u32 {
    let mut i = 4;
    let mut last_four: Vec<char> = Vec::new();
    for c in inp.chars().take(3) {
        last_four.push(c);
    }
    for c in inp.chars().skip(3) {
        last_four.push(c);
        if last_four.iter().map(|x| *x).collect::<HashSet<char>>().iter().count() == 4 {
            break
        }
        last_four.remove(0);
        i += 1;
    }
    i
}

fn part_2(inp: &str) -> u32 {
    let mut i = 14;
    let mut last_four: Vec<char> = Vec::new();
    for c in inp.chars().take(13) {
        last_four.push(c);
    }
    for c in inp.chars().skip(13) {
        last_four.push(c);
        if last_four.iter().map(|x| *x).collect::<HashSet<char>>().iter().count() == 14 {
            break
        }
        last_four.remove(0);
        i += 1;
    }
    i
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
