use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/six/input.in");
    //let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    let chars : Vec<char> = input.chars().collect();
    println!("Part 1: {}", find_marker(&chars, 4));

    println!("Part 2: {}", find_marker(&chars, 14));
}

fn is_marker(b : &Vec<char>) -> bool {
    return b.into_iter().cloned().collect::<HashSet<char>>().len() == b.len()
}

fn find_marker(input : &Vec<char>, size : usize) -> usize {
    let mut buffer : Vec<char> = input[0..size].to_vec();
    let mut processed = size;
    for c in input[size..].into_iter() {
        if is_marker(&buffer) {
            break;
        }
        buffer.remove(0); // Popping last
        buffer.push(*c);
        processed += 1;
    }
    println!("{}", buffer.into_iter().collect::<String>());
    processed
}
