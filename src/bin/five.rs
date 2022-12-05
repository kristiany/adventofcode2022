fn main() {
    let input = include_str!("../inputs/five/input.in");
    let split = input.split("\n").into_iter();
    let mut moves = Vec::new();
    let mut stacks: [Vec<char>; 9] = Default::default();
    for row in split {
        if row.contains("[") {
            let chars : Vec<char> = row.chars().collect();
            for i in (1..chars.len()).step_by(4) {
                //println!("{} {}", i, chars[i]);
                if chars[i] != ' ' {
                    stacks[i / 4].push(chars[i]);
                }
            }
        }
        else if row.starts_with("move") {
            let mut move_input = row.split(" ");
            moves.push(Move {
                count: move_input.nth(1).unwrap().parse::<usize>().unwrap(),
                from: move_input.nth(1).unwrap().parse::<usize>().unwrap(),
                to: move_input.nth(1).unwrap().parse::<usize>().unwrap(),
            })
        }
    }
    // Turn them stacks
    stacks.iter_mut().for_each(| s | { s.reverse(); } );

    let mut workstack1 : Vec<Vec<char>> = stacks.clone()
        .map(| v | v.clone().to_vec())
        .to_vec();
    //println!("{:?}", stacks);
    for m in &moves {
        for _ in 1..(m.count + 1) {
            let to_move = workstack1[m.from - 1].pop().unwrap();
            workstack1[m.to - 1].push(to_move);
            //println!("Pushed {to_move}");
        }
        //println!("{:?}: {:?}", m, stacks);
    }

    println!("Part 1: {}", top(workstack1));

    let mut workstack2 : Vec<Vec<char>> = stacks.clone()
        .map(| v | v.clone().to_vec())
        .to_vec();
    for m in &moves {
        // TODO Come on, why is this so hard?
        /*let from_size = workstack2[m.from - 1].len();
        let to_move = workstack2[m.from - 1]
            .drain(from_size - m.count - 1..).into_iter();*/
        // Giving up, let's use pop!
        let mut to_moves = Vec::new();
        for _ in 1..(m.count + 1) {
            to_moves.push(workstack2[m.from - 1].pop().unwrap());
        }
        to_moves.reverse();

        workstack2[m.to - 1].extend(to_moves);
    }

    println!("Part 2: {}", top(workstack2));
}

fn top(stacks : Vec<Vec<char>>) -> String {
    stacks.into_iter()
        .filter(| s | ! s.is_empty())
        .map(| s | *s.last().unwrap())
        .collect::<String>()
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}