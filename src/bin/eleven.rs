fn main() {
    let input = include_str!("../inputs/eleven/input.in");

    let mut items : Vec<Vec<u128>> = Vec::new();
    let mut ops : Vec<MonkOp> = Vec::new();
    for monkey_input in input.split("Monkey").into_iter() {
        if monkey_input.is_empty() {
            continue;
        }
        //println!("monkey{}", monkey_input);
        let rows : Vec<&str> = monkey_input.split("\n").into_iter().collect();
        let mut list_input : Vec<u128> = rows[1].split(":").into_iter()
            .collect::<Vec<&str>>()[1].trim()
            .split(",")
            .map(| i | i.trim().parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        list_input.reverse();  // Make it stack-like so we can pop!
        items.push( list_input.to_owned());
        let ops_input : Vec<&str> = rows[2].split("old").into_iter().collect::<Vec<&str>>()[1].trim()
            .split(" ").into_iter().collect();
        //println!("ops in {:?}", ops_input);
        let op = if ops_input[0] == "+" { Op::ADD } else { Op::MUL };
        let op_value = if ops_input.len() > 1 {
            Some(ops_input[1].parse::<u128>().unwrap())
        } else {
            None // means the value itself
        };
        let test = rows[3].split("by").into_iter().collect::<Vec<&str>>()[1]
            .trim().parse::<u128>().unwrap();
        let if_true = rows[4].split("monkey").into_iter().collect::<Vec<&str>>()[1]
            .trim().parse::<usize>().unwrap();
        let if_false = rows[5].split("monkey").into_iter().collect::<Vec<&str>>()[1]
            .trim().parse::<usize>().unwrap();
        ops.push(MonkOp {
            op: op,
            op_value: op_value,
            test: test,
            if_true: if_true,
            if_false: if_false,
        })
    }
    println!("{:?}", items);
    println!("{:?}", ops);

    let mut inspections = Vec::new();
    for _ in 0..items.len() {
        inspections.push(0usize);
    }
    let mut part1_items = items.clone();
    for round in 1..21 {
        for turn in 0..part1_items.len() {
            let op = &ops[turn];
            inspections[turn] += part1_items[turn].len();
            while let Some(item) = part1_items[turn].pop() {
                let value = op.op_value.unwrap_or_else(|| item );
                let after_op = if op.op == Op::ADD { item + value } else { item * value };
                let level = after_op / 3;
                let throw_to = if level % op.test == 0 {
                    op.if_true
                }
                else {
                    op.if_false
                };
                part1_items[throw_to].insert(0, level);
            }
        }
    }
    let mut top = inspections.clone();
    top.sort();
    top.reverse();
    println!("Part 1: {}", top[0] * top[1]);

    let mut part2_items = items.clone();
    inspections.clear();
    for _ in 0..part2_items.len() {
        inspections.push(0usize);
    }
    let common_divisor = &ops.iter()
        .map(| op | op.test)
        .reduce(| a, b | a * b)
        .unwrap();
    for round in 1..10001 {
        //println!("Round {round}");
        for turn in 0..part2_items.len() {
            let op = &ops[turn];
            inspections[turn] += part2_items[turn].len();
            while let Some(item) = part2_items[turn].pop() {
                let value = op.op_value.unwrap_or_else(|| item );
                let level = if op.op == Op::ADD { item + value } else { item * value }
                    % common_divisor;
                let throw_to = if level % op.test == 0 {
                    op.if_true
                }
                else {
                    op.if_false
                };
                part2_items[throw_to].insert(0, level);
            }
        }
        //println!("{:?}", inspections);
    }
    let mut top = inspections.clone();
    top.sort();
    top.reverse();
    println!("Part 2: {}", top[0] * top[1]);

}

#[derive(PartialEq, Debug)]
enum Op { MUL, ADD }

#[derive(Debug)]
struct MonkOp {
    op: Op,
    op_value: Option<u128>,
    test: u128,
    if_true: usize,
    if_false: usize,
}