use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/ten/input.in");
    let ins : Vec<Option<i32>> = input.split("\n")
        .into_iter()
        .map(| i | {
            if i.starts_with("addx") {
                return Some(i.split(" ").into_iter()
                    .collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap());
            }
            None
        })
        .collect();

    let mut x: i32 = 1;
    let mut cycle: usize = 1;
    let mut signals: i32 = 0;
    let sig_cycles : HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);
    for c in &ins {
        if c.is_some() {
            //println!("Cycle(mid addx): {}, val: {}, strength: {}", cycle, x, cycle as i32 * x);
            if sig_cycles.contains(&cycle) {
                signals += cycle as i32  * x;
            }
            cycle += 1;
        }
        if sig_cycles.contains(&cycle) {
            signals += cycle as i32  * x;
        }
        //println!("Cycle: {}, val: {}, strength: {}", cycle, x, cycle as i32  * x);
        if c.is_some() {
            x += c.unwrap();
        }
        cycle += 1;
    }

    println!("Part 1: {}", signals);

    x = 1;
    cycle = 1;
    let mut screen : [[char; 40]; 6] =  [['.'; 40]; 6];
    for c in &ins {

        if c.is_some() {
            print_screen(cycle, x, &mut screen);
            cycle += 1;
        }
        print_screen(cycle, x, &mut screen);
        if c.is_some() {
            x += c.unwrap();
        }
        cycle += 1;
    }

    println!("Part 2");
    for s in screen {
        println!("{:?}", s.into_iter().collect::<String>());
    }
}

fn print_screen(cycle: usize, x: i32, screen : &mut [[char; 40]; 6]) {
    // x is 1-based
    // sprite pos is 0-based
    if cycle as i32 % 40 >= x && cycle as i32 % 40 <= x + 2 {
        let row = (cycle - 1) / 40;
        let col = (cycle - 1) - row * 40;
        screen[row][col] = '#';
    }
}