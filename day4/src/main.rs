use std::{fs};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 2)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 4)
}

type Pairs = Vec<((u32, u32), (u32, u32))>;

fn handle_input(file: &str) -> Pairs {
    fs::read_to_string(file).unwrap()
    .lines()
    .map(|x| {
        let mut split = x.splitn(2, ",");
        let (a, b) = (split.next().unwrap(), split.next().unwrap());
        let mut asplit = a.splitn(2, "-");
        let (a1, a2): (u32, u32) = (asplit.next().unwrap().parse().unwrap(), asplit.next().unwrap().parse().unwrap());
        let mut bsplit = b.splitn(2, "-");
        let (b1, b2) = (bsplit.next().unwrap().parse().unwrap(), bsplit.next().unwrap().parse().unwrap());
        ((a1, a2), (b1, b2))
    }).collect()
}


fn part_1(inp: &Pairs) -> usize {
    inp
        .into_iter()
        .filter(|((a1, a2), (b1, b2))| {
            (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
        })
        .count()
}

fn part_2(inp: &Pairs) -> usize {
    inp
        .into_iter()    
        .filter(|((a1, a2),(b1, b2))| {
            (a1 <= b1 && a2 >= b1) || (b1 <= a1 && b2 >= a1)
        })
        .count()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
