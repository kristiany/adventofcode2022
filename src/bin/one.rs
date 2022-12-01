fn main() {
    let input = include_str!("../inputs/one/input.in");

    let mut start = Vec::new();
    start.push(Vec::new());
    let mut res: Vec<u32> = input.split('\n').into_iter()
        .fold(start, | mut acc, x| {
            if x == "" { acc.push(Vec::new()); }
            else {
                acc.last_mut().unwrap()
                    .push(x.parse::<u32>().unwrap());
            }
            acc
        }).into_iter()
        .map(| v | v.into_iter().sum())
        .collect();
    res.sort();
    println!("Part 1: {}", res.last().unwrap());
    println!("Part 2: {}", res.as_slice()[res.len() - 3..].to_vec().into_iter().sum::<u32>());
}
