use std::collections::HashMap;
use std::path::PathBuf;

fn parse_terminal_log(log: &str) -> (i32, i32) {
    let mut pwd = PathBuf::new();
    pwd.push("/");

    let mut file_system = HashMap::<String, i32>::new();
    let mut dir_sizes = HashMap::<String, i32>::new();

    dir_sizes.insert("/".to_string(), 0);

    for line in log.lines() {
        if line == "$ cd .." {
            pwd.pop();
        } else if line == "$ cd /" {
            pwd = PathBuf::from("/");
        } else if line.starts_with("$ cd ") {
            pwd.push(line.strip_prefix("$ cd ").unwrap());
        } else if line == "$ ls" {
            continue;
        } else {
            let (size, name) = line
                .split_once(' ')
                .map(|(size, name)| (size.parse::<i32>().unwrap_or(0), name))
                .unwrap();
            let path = pwd.join(name);
            let path_str = path.to_str().unwrap();
            file_system.insert(path_str.to_string(), size);

            if size == 0 {
                dir_sizes.insert(path_str.to_string(), 0);
            }
        }
    }

    for (k, v) in dir_sizes.iter_mut() {
        for (path, size) in &file_system {
            let result = PathBuf::from(path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(k);
            if result {
                *v += size;
            }
        }
    }

    let limit = 100000;
    let total = dir_sizes.values().filter(|v| **v <= limit).sum();

    let total_space = 70000000;
    let min_space = 30000000;
    let used_space = total_space - dir_sizes["/"];
    let target_dir_size = dir_sizes
        .values()
        .filter(|v| (used_space + *v) >= min_space)
        .min()
        .unwrap()
        .clone();

    return (total, target_dir_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_path_test() {
        let path = "/b.txt";
        let pathbuf = PathBuf::from(path);
        let tmp = pathbuf.parent().unwrap().to_str().unwrap();
        assert_eq!("/", tmp);
    }

    #[test]
    fn example_part1() {
        let console_log = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!((95437, 24933642), parse_terminal_log(console_log));
    }
}

pub fn day07() {
    let _input = include_str!("input.txt");

    println!("Day07 answers: {:?}", parse_terminal_log(_input)); // 1453349.... part2 22948823
}
