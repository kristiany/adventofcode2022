use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../inputs/fourteen/input.in");
    let lines = input.split("\n").into_iter()
        .flat_map(| line_input | Line::parse(line_input))
        .collect::<Vec<Line>>();

    let mut cur_sand = (500, 0);
    let mut sands: HashSet<Pos> = HashSet::new();
    let abyss = lines.iter()
        .map(| l | std::cmp::max(l.start.1, l.end.1))
        .max()
        .unwrap();
    println!("abyss: {}", abyss);
    loop {
        if cur_sand.1 > abyss {
            // Gone in the abyss, all done
            //cur_sand = (500, 0);
            break;
        }
        // Check downward
        if !collision(&(cur_sand.0, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0, cur_sand.1 + 1);
            //println!(" dropping down {:?}", cur_sand);
        }
        // Check down left
        else if !collision(&(cur_sand.0 - 1, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0 - 1, cur_sand.1 + 1);
            //println!(" dropping left {:?}", cur_sand);
        }
        // Check down right
        else if !collision(&(cur_sand.0 + 1, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0 + 1, cur_sand.1 + 1);
            //println!(" dropping right {:?}", cur_sand);
        }
        // Hit a resting place
        else {
            sands.insert(cur_sand.clone());
            //println!(" at rest {:?}", cur_sand);
            cur_sand = (500, 0);
        }
    }

    println!("Part 1: {}", sands.len());

    let start = Instant::now();
    let floor = abyss + 2;
    let mut cur_sand = (500, 0);
    loop {
        // Check downward
        let not_on_floor = cur_sand.1 + 1 != floor;
        if not_on_floor && !collision(&(cur_sand.0, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0, cur_sand.1 + 1);
        }
        // Check down left
        else if not_on_floor && !collision(&(cur_sand.0 - 1, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0 - 1, cur_sand.1 + 1);
        }
        // Check down right
        else if not_on_floor && !collision(&(cur_sand.0 + 1, cur_sand.1 + 1), &lines, &sands) {
            cur_sand = (cur_sand.0 + 1, cur_sand.1 + 1);
        }
        // Hit a resting place
        else {
            sands.insert(cur_sand.clone());
            if cur_sand == (500, 0) {
                break; // Reached the top!
            }
            //println!(" at rest {:?}", cur_sand);
            cur_sand = (500, 0);
        }
    }

    println!("Time: {}", start.elapsed().as_secs());
    println!("Part 2: {}", sands.len());
}

fn collision(p : &Pos, lines : &Vec<Line>, sands : &HashSet<Pos>) -> bool {
    if sands.contains(p) {
        return true;
    }
    for line in lines {
        // filter out lines that aren't close -- actually increases exec time!
        //.iter().filter(| l | ! (l.end.0 < p.0 || l.start.0 > p.0 || l.end.1 < p.1 || l.start.1 > p.1)) {
        if line.collide(p) {
            return true;
        }
    }
    false
}

type Pos = (usize, usize);

#[derive(Clone, Debug)]
struct Line {
    start: Pos,
    end: Pos
}

impl Line {
    fn collide(&self, p : &Pos) -> bool {
        let start = self.start;
        let end = self.end;
        //println!(" p {:?}, line {:?}", p, self);
        p.0 == start.0 && p.1 >= start.1 && p.1 <= end.1
        || p.1 == start.1 && p.0 >= start.0 && p.0 <= end.0
    }

    fn parse(input : &str) -> Vec<Line> {
        let points : Vec<Pos> = input.split(" -> ").into_iter()
            .map(| pin | {
                let pair = pin.split(",").into_iter().collect::<Vec<&str>>();
                (pair[0].parse::<usize>().unwrap(), pair[1].parse::<usize>().unwrap())
            })
            .collect();
        points.windows(2).map(| ps |
                // Order by lowest first
                if ps[0].0 == ps[1].0 && ps[0].1 < ps[1].1
                    || ps[0].1 == ps[1].1 && ps[0].0 < ps[1].0 {
                    Line { start: ps[0], end: ps[1] }
                } else {
                    Line { start: ps[1], end: ps[0] }
                }
            )
            .into_iter().collect()
    }
}