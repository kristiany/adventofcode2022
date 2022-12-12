use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

fn main() {
    let input = include_str!("../inputs/twelve/input.in");
    let map : Vec<Vec<char>> = input.split("\n").into_iter()
        .map(| r | r.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    //println!("{:?}", map);
    let w = &map[0].len();
    let h = &map.len();
    let (start, end) = find_start_end(&map, *w, *h);
    //println!("start: {:?}, end: {:?}", start, end);

    let path = astar(&map, *w, *h, start, end).unwrap();
    //println!("{:?}", path);
    println!("Part 1: {} steps", path.len() as i32 - 1);

    let starts = find_starts(&map, *w, *h);
    println!("Number of starts {}", starts.len());
    let mut shortest = usize::MAX;
    let mut nr = 1;
    for start in starts {
        println!("Testing start nr {nr}");
        let result = astar(&map, *w, *h, start, end);
        if result.is_some() {
            let len = result.unwrap().len();
            if len - 1 < shortest {
                shortest = len - 1;
            }
        }
        nr += 1;
    }
    println!("Part 2: {} steps", shortest);

}
// https://en.wikipedia.org/wiki/A*_search_algorithm
fn astar(map : &Vec<Vec<char>>, w: usize, h: usize, start: Pos, end: Pos) -> Option<Vec<Pos>> {
    // A* \o/
    let mut queue : Vec<Pos> = Vec::new();
    queue.push(start);
    let mut path: HashMap<Pos, Pos> = HashMap::new();
    let mut gscore: HashMap<Pos, i32> = HashMap::new();
    gscore.insert(start, 0);
    let mut fscore: HashMap<Pos, i32> = HashMap::new();
    fscore.insert(start, heur(&start, &end));
    //let mut visited: HashSet<Pos> = HashSet::new();
    //let mut visited_cs: HashSet<char> = HashSet::new();
    //visited_cs.insert('a');

    while let Some(cur) = queue.pop() {
        if cur == end {
            return Some(backtrack(&path, &cur));
        }

        let mut neighbors : Vec<Pos> = Vec::new();
        if cur.0 > 0 {
            neighbors.push((cur.0 - 1, cur.1));
        }
        if cur.0 < w - 1 {
            neighbors.push((cur.0 + 1, cur.1));
        }
        if cur.1 > 0 {
            neighbors.push((cur.0, cur.1 - 1));
        }
        if cur.1 < h - 1 {
            neighbors.push((cur.0, cur.1 + 1));
        }
        for p in neighbors
                .into_iter()
                .filter(| p| valid_alpha(p, &cur, &map))
                .collect::<Vec<Pos>>() {
            //println!("      Cur {:?}, next {:?}", map[cur.1][cur.0], map[p.1][p.0]);
            let tentative_gscore = gscore[&cur] + 1;
            if tentative_gscore < *gscore.get(&p).unwrap_or(&i32::MAX) {
                path.insert(p,cur);
                gscore.insert(p, tentative_gscore);
                fscore.insert(p, tentative_gscore + heur(&p, &end));
                if !queue.contains(&p) {
                    insert_sorted(&p, &mut queue, &fscore);
                }
            }
            //visited.insert(p);
            //visited_cs.insert(map[p.1][p.0]);
        }
    }
    // Debug visited positions
    /*for y in 0..h {
        for x in 0..w {
            if visited.contains(&(x, y)) {
                print!("x");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!("{:?}", visited_cs);*/
    None
}

fn valid_alpha(to: &Pos, cur: &Pos, map : &Vec<Vec<char>>) -> bool {
    let cur_char = if map[cur.1][cur.0] == 'S' { 'a' }
        else if map[cur.1][cur.0] == 'E' { 'z' }
        else { map[cur.1][cur.0] };
    let p_char = if map[to.1][to.0] == 'E' { 'z' }
        else if map[to.1][to.0] == 'S' { 'a' }
        else { map[to.1][to.0] };
    cur_char as u32 == p_char as u32
        || cur_char as u32 + 1 == p_char as u32
        || p_char as u32 <= cur_char as u32
}

fn heur(from : &Pos, to : &Pos) -> i32 {
    // Manhattan distance
    (to.0 as i32 - from.0 as i32).abs() + (to.1 as i32 - from.1 as i32).abs()
}

fn backtrack(path : &HashMap<Pos, Pos>, cur: &Pos) -> Vec<Pos> {
    let mut result : Vec<Pos> = Vec::new();
    let mut next = cur;
    result.push(*cur);
    while path.contains_key(&next) {
        next = &path[&next];
        result.push(*next);
    }
    result
}

fn find_start_end(map : &Vec<Vec<char>>, w: usize, h: usize) -> (Pos, Pos){
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == 'S' {
                start = (x, y);
            }
            else if map[y][x] == 'E' {
                end = (x, y);
            }
        }
    }
    (start, end)
}

fn find_starts(map : &Vec<Vec<char>>, w: usize, h: usize) -> Vec<Pos> {
    let mut result : Vec<Pos> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == 'a' {
                result.push((x, y));
            }
        }
    }
    result
}

fn insert_sorted(p : &Pos, vec : &mut Vec<Pos>, fscore : &HashMap<Pos, i32>) {
    let score = fscore.get(p).unwrap_or(&i32::MAX);
    for (i, v) in vec.clone().into_iter().enumerate() {
        if score >= fscore.get(&v).unwrap_or(&i32::MAX) {
            &mut vec.insert(i, *p);
            return;
        }
    }
    &mut vec.insert(0, *p);
}