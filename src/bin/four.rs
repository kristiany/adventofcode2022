use std::str::Split;

fn main() {

    fn count_overlaps(input: Split<&str>, p: fn((u32, u32), (u32, u32)) -> bool) -> usize {
        input.map(| row |
                row.split(",")
                    .map(| sched | {
                        let range_input : Vec<&str> = sched.split("-").collect();
                        (range_input[0].parse::<u32>().unwrap(),
                         range_input[1].parse::<u32>().unwrap())
                    }).collect())
            .into_iter()
            .filter(| pair : &Vec<(u32, u32)> | {
                let a = pair[0];
                let b = pair[1];
                p(a, b)
            })
            .count()
    }

    let input = include_str!("../inputs/four/input.in").split("\n").into_iter();
    let result = count_overlaps(input.clone(),
    | a, b | a.0 >= b.0 && a.1 <= b.1 || b.0 >= a.0 && b.1 <= a.1
    );
    println!("Part 1: {result}");

    let result2 = count_overlaps(input.clone(),
     | a, b |
         a.0 >= b.0 && a.0 <= b.1
         || a.1 >= b.0 && a.1 <= b.1
         || b.0 >= a.0 && b.0 <= a.1
         || b.1 >= a.0 && b.1 <= a.1);
    println!("Part 2: {result2}");
}