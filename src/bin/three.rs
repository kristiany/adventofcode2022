use std::collections::HashSet;

fn main() {

    fn to_prio(c: char) -> u32 {
        (c as u32) - (if c.is_lowercase() { 96 } else { 64 - 26 })
    }

    let input = include_str!("../inputs/three/input.in").split("\n").into_iter();
    let result : u32 = input.clone()
        .map(| i | {
            let half = i.len() / 2;
            (HashSet::from_iter(
                i[0..half].trim().chars()),
             HashSet::from_iter(
                 i[half..].trim().chars()))
        })
        .map(| b : (HashSet<char>, HashSet<char>) | {
            b.0.intersection(&b.1)
                .map(| c | {
                    //print!("{c} to {}, ", to_prio(*c));
                    to_prio(*c)
                })
                .sum::<u32>()
        })
        .sum();
    println!("Part 1: {result}");

    let result2 : u32 = input.clone().collect::<Vec<&str>>()
        .chunks(3)
        .map(| group | {
            group.into_iter()
                .map(| r | HashSet::from_iter(r.trim().chars()))
                .reduce(| acc : HashSet<char>, b | {
                    acc.intersection(&b).cloned().collect()
                })
                .map(| cset | {
                    //print!("{c} to {}, ", to_prio(*c));
                    cset.into_iter()
                        .map(| c | to_prio(c))
                        .sum::<u32>()
                }).into_iter()
                .sum::<u32>()
        })
        .sum();
    println!("Part 2: {result2}");
}