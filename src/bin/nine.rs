use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/nine/input.in");

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head_current = (0, 0);
    let mut tail_current = (0, 0);
    tail_visited.insert(tail_current);
    for ins in input.split("\n").into_iter() {
        let parts : Vec<&str> = ins.split(" ").into_iter().collect();
        let dir = parts[0].trim();
        let steps = parts[1].trim().parse::<usize>().unwrap();
        if dir == "R" {
            for _ in 0..steps {
                head_current = (head_current.0 + 1, head_current.1);
                tail_current = move_tail(tail_current, head_current);
                tail_visited.insert(tail_current);
            }
        }
        else if dir == "U" {
            for _ in 0..steps {
                head_current = (head_current.0, head_current.1 - 1);
                tail_current = move_tail(tail_current, head_current);
                tail_visited.insert(tail_current);
            }
        }
        else if dir == "L" {
            for _ in 0..steps {
                head_current = (head_current.0 - 1, head_current.1);
                tail_current = move_tail(tail_current, head_current);
                tail_visited.insert(tail_current);
            }
        }
        else if dir == "D" {
            for _ in 0..steps {
                head_current = (head_current.0, head_current.1 + 1);
                tail_current = move_tail(tail_current, head_current);
                tail_visited.insert(tail_current);
            }
        }
    }

    println!("{:?}", tail_visited);
    println!("Part 1: {}", tail_visited.len());

    tail_visited.clear();
    let mut rope : [(i32, i32); 10] = Default::default();
    tail_visited.insert(rope[9]);
    for ins in input.split("\n").into_iter() {
        let parts : Vec<&str> = ins.split(" ").into_iter().collect();
        let dir = parts[0].trim();
        let steps = parts[1].trim().parse::<usize>().unwrap();
        if dir == "R" {
            for _ in 0..steps {
                rope[0] = (rope[0].0 + 1, rope[0].1);
                for i in 1..10 {
                    rope[i] = move_tail(rope[i], rope[i - 1]);
                }
                tail_visited.insert(rope[9]);
            }
        }
        else if dir == "U" {
            for _ in 0..steps {
                rope[0] = (rope[0].0, rope[0].1 - 1);
                for i in 1..10 {
                    rope[i] = move_tail(rope[i], rope[i - 1]);
                }
                tail_visited.insert(rope[9]);
            }
        }
        else if dir == "L" {
            for _ in 0..steps {
                rope[0] = (rope[0].0 - 1, rope[0].1);
                for i in 1..10 {
                    rope[i] = move_tail(rope[i], rope[i - 1]);
                }
                tail_visited.insert(rope[9]);
            }
        }
        else if dir == "D" {
            for _ in 0..steps {
                rope[0] = (rope[0].0, rope[0].1 + 1);
                for i in 1..10 {
                    rope[i] = move_tail(rope[i], rope[i - 1]);
                }
                tail_visited.insert(rope[9]);
            }
        }
    }

    println!("{:?}", tail_visited);
    println!("Part 2: {}", tail_visited.len());
}

fn move_tail(tail : (i32, i32), head_to : (i32, i32)) -> (i32, i32) {
    let mut dist_x = head_to.0 - tail.0;
    let mut dist_y = head_to.1 - tail.1;
    if dist_x.abs() < 2 && dist_y.abs() < 2 {
        //println!("no head: {:?}, tail: {:?}, dist {:?}", head_to, tail, (dist_x, dist_y));
        return tail;
    }
    //println!("move head: {:?}, dist {:?}", head_to, (dist_x, dist_y));
    // Clamp/Normalize
    if dist_x.abs() > 1 {
        dist_x = dist_x / dist_x.abs();
    }
    if dist_y.abs() > 1 {
        dist_y = dist_y / dist_y.abs();
    }
    //println!("    clamped dist {:?}, tail: {:?}", (tail.0 + dist_x, tail.1 + dist_y), (dist_x, dist_y));
    (tail.0 + dist_x, tail.1 + dist_y)
}