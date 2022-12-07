extern crate core;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/seven/input.in");

    let mut current : Vec<&str> = Vec::new();
    current.push("/");

    let mut files: HashMap<Vec<&str>, Vec<File>> = HashMap::new();
    let mut it = input.split("\n").into_iter();
    let mut ls_mode = false;
    while let Some(entry) = it.next() {
        if entry.starts_with("$") {
            ls_mode = false;
        }
        if ls_mode {
            if entry.starts_with("dir") {
                continue;  // Ignore
            }
            else {
                let file_info = entry.trim().split(" ").collect::<Vec<&str>>();
                let key = current.clone();
                if !files.contains_key(&key) {
                    files.insert(key.clone(), Vec::new());
                }
                files.get_mut(&key).unwrap().push(File {
                    fname: file_info[1],
                    fsize: file_info[0].parse::<usize>().unwrap()
                });
            }

        }
        if entry == "$ ls" {
            ls_mode = true;
        }
        else if entry.starts_with("$ cd") {
            let name = entry.trim().split(" ").collect::<Vec<&str>>()[2];
            if name == ".." {
                current.pop();
            }
            else if name == "/" {
                current.clear();
                current.push("/");
            }
            else {
                current.push(name);
            }
        }
    }

    let mut sizes: HashMap<Vec<&str>, usize> = HashMap::new();
    for key in files.keys() {
        let dir_size = files.get(key).unwrap().into_iter()
            .map(| f | f.fsize)
            .sum();
        let mut path = key.clone();
        while path.len() > 0 {
            let size_key = path.clone();
            if sizes.contains_key(&size_key) {
                let val = sizes.get(&size_key).unwrap();
                sizes.insert(size_key, *val + dir_size);
            }
            else {
                sizes.insert(size_key, dir_size);
            }
            path.pop();
        }
    }

    println!("{:?}", files);
    println!("{:?}", sizes);
    println!("Part 1: {}", sizes.values().filter(| v | **v <= 100_000usize).sum::<usize>());

    let unused = 70_000_000usize - sizes.get(&Vec::from(["/"])).unwrap();
    let need = 30_000_000usize - unused;
    let min = sizes.values()
        .filter(| v | **v > need )
        .min()
        .unwrap();
    println!("Part 2: {min}");
}

#[derive(Debug)]
struct File<'a> {
    fname: &'a str,
    fsize: usize
}
