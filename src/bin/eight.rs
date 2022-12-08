use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/eight/input.in");

    let rows: Vec<Vec<u32>> = input.split("\n")
        .map(| row | row.chars().map(| c | c.to_digit(10).unwrap()).collect())
        .into_iter().collect();
    let w = rows[0].len();
    let h = rows.len();
    let mut visibles: HashSet<(usize, usize)> = HashSet::new();
    for y in 1..h - 1 {
        let row = &rows[y];
        let mut max = 0;
        for x in 1..w - 1 {
            if row[x - 1] > max {
                max = row[x - 1];
            }
            if row[x] > max {
                visibles.insert((x, y));
            }
        }
        let mut max = 0;
        for x in (1..w - 1).rev() {
            if row[x + 1] > max {
                max = row[x + 1];
            }
            if row[x] > max {
                visibles.insert((x, y));
            }
        }
    }
    // y-wise
    for x in 1..w - 1 {
        let column: Vec<u32> = rows.clone().into_iter().map(|v| v[x]).collect();
        let mut max = 0;
        for y in 1..h - 1 {
            if column[y - 1] > max {
                max = column[y - 1];
            }
            if column[y] > max {
                visibles.insert((x, y));
            }
        }
        let mut max = 0;
        for y in (1..h - 1).rev() {
            if column[y + 1] > max {
                max = column[y + 1];
            }
            if column[y] > max {
                visibles.insert((x, y));
            }
        }
    }

    println!("{:?}", visibles);
    let edge = w * 2 + h * 2 - 4; // Corners are counted twice
    println!("Part 1: mid: {} + edge {} = {}", visibles.len(), edge, visibles.len() + edge);

    let mut highest = 0;
    for y in 0..h {
        for x in 0..w {
            let score = scenic_score(x, y, &rows, w, h);
            if score > highest {
                highest = score;
            }
         }
    }
    println!("Part 2: {}", highest);
}

// ! The edge should possibly not be in this score but it works :shrugging:
fn scenic_score(treex: usize, treey: usize, rows: &Vec<Vec<u32>>, w: usize, h: usize) -> u32 {
    let mut upscore = 0;
    let mut downscore = 0;
    let mut leftscore = 0;
    let mut rightscore = 0;
    let tree = rows[treey][treex];
    // Up
    if treey > 0 {
        for y in (0..treey).rev() {
            upscore += 1;
            if tree <= rows[y][treex] {
                break;
            }
        }
    }
    // Down
    if treey < h - 1 {
        for y in treey + 1..h {
            downscore += 1;
            if tree <= rows[y][treex] {
                break;
            }
        }
    }
    // Left
    if treex > 0 {
        for x in (0..treex).rev() {
            leftscore += 1;
            if tree <= rows[treey][x] {
                break;
            }
        }
    }
    // Right
    if treex < w - 1 {
        for x in treex + 1..w {
            rightscore += 1;
            if tree <= rows[treey][x] {
                break;
            }
        }
    }
    println!("({}, {}): up {}, down {}, left {}, right {}", treex, treey,
             upscore, downscore, leftscore, rightscore);
    if upscore == 0 {
        upscore = 1;
    }
    if downscore == 0 {
        downscore = 1;
    }
    if leftscore == 0 {
        leftscore = 1;
    }
    if rightscore == 0 {
        rightscore = 1;
    }
    println!("({}, {}) -> score {}", treex, treey, upscore * downscore * leftscore * rightscore);
    upscore * downscore * leftscore * rightscore
}
