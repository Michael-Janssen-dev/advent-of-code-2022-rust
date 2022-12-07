use std::{fs, collections::HashMap};

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 95437)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 24933642)
}

fn handle_input(file: &str) -> Vec<Command> {
    let inp = fs::read_to_string(file).unwrap();
    inp.split("$").into_iter().skip(1).map(|x| {
        let mut lines = x.lines();
        let command = lines.next().unwrap().trim();
        match command.get(..2) {
            Some("ls") => {
                let mut files: Vec<FileSystem> = Vec::new();
                for line in lines {
                    let (a, _) = line.split_once(' ').unwrap();
                    if a == "dir" {
                        files.push(FileSystem::Folder)
                    } else {
                        files.push(FileSystem::File(a.parse().unwrap()));
                    }
                }
                Command::Ls(files)
            },
            Some("cd") => {
                let folder = command.get(3..).unwrap();
                if folder == ".." {
                    return Command::Cd(Cd::Out)
                } else {
                    return Command::Cd(Cd::In(folder.to_string()))
                }
            }
            _ => panic!("Huh")
        }
    }).collect()
}

enum FileSystem {
    Folder,
    File(u32)
}

enum Cd {
    In(String),
    Out
}

enum Command {
    Ls(Vec<FileSystem>),
    Cd(Cd)
}

fn folder_sizes(commands: &Vec<Command>) -> HashMap<Vec<String>, u32> {
    let mut path: Vec<String> = Vec::new();
    let mut sizes: HashMap<Vec<String>, u32> = HashMap::new();

    for command in commands {
        match command {
            Command::Ls(x) => {
                let size: u32 = x.iter().map(|x| if let FileSystem::File(n) = x {*n} else {0}).sum();
                (0..path.len()).for_each(|i| {
                    sizes.entry(path[..=i].to_vec()).and_modify(|x| *x += size).or_insert(size);
                });
            },
            Command::Cd(x) => {
                match x {
                    Cd::In(y) => {
                        path.push(y.to_owned());
                    }
                    Cd::Out => {
                        path.pop();
                    }
                }
            }
            
        }
    }

    sizes
}


fn part_1(inp: &Vec<Command>) -> u32 {
    folder_sizes(inp)
        .values()
        .filter(|x| **x <= 100000)
        .sum()
}


fn part_2(inp: &Vec<Command>) -> u32 {
    let sizes = folder_sizes(inp);
    let unused_space = 70000000 - sizes.get(&vec!["/".to_string()]).unwrap();
    let required_space = 30000000 - unused_space;

    folder_sizes(inp)
        .values()
        .filter(|x| **x >= required_space)
        .min()
        .unwrap()
        .to_owned()
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
