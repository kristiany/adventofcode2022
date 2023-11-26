use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/seventeen/input.in");
    let gas = input.trim().chars().into_iter()
        .map(|s| if s == '<' { -1 } else { 1 })
        .collect::<Vec<i8>>();

    // local origo left down
    //  ####
    let a = Rock {
        ps: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        bbox: ((0, 0), (3, 0)),
    };
    /*
        .#.
        ###
        .#.
    */
    let b = Rock {
        ps: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        bbox: ((0, 0), (2, 2)),
    };
    /*
        ..#
        ..#
        ###
    */
    let c = Rock {
        ps: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        bbox: ((0, 0), (2, 2)),
    };
    /*
        #
        #
        #
        #
    */
    let d = Rock {
        ps: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        bbox: ((0, 0), (0, 3)),
    };
    /*
        ##
        ##
     */
    let e = Rock {
        ps: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        bbox: ((0, 0), (1, 1)),
    };
    let rocs = vec![a, b, c, d, e];
    let mut roc_index = 0;
    let mut gas_index = 0;
    let mut tunnel: Vec<RockPos> = Vec::new();
    let mut y_start = 0;

    println!("Gas length {}", gas.len());

    for _r in 1..2023 {
        y_start += 3;
        let rock = &rocs[roc_index];
        let mut falling_rock = RockPos {
            pos: (2, y_start),
            rref: rock,
        };
        loop {
            // Gas stream
            let current_gas = gas[gas_index];
            gas_index = (gas_index + 1) % gas.len();
            if current_gas < 0 && falling_rock.pos.0 > 0
                && !collision(&tunnel,
                              &RockPos {
                                  rref: falling_rock.rref,
                                  pos: (falling_rock.pos.0 - 1, falling_rock.pos.1),
                              }) {
                falling_rock.pos.0 = falling_rock.pos.0 - 1;
            } else if current_gas > 0 && falling_rock.pos.0 + falling_rock.rref.bbox.1.0 < 6
                && !collision(&tunnel,
                              &RockPos {
                                  rref: falling_rock.rref,
                                  pos: (falling_rock.pos.0 + 1, falling_rock.pos.1),
                              }) {
                falling_rock.pos.0 = falling_rock.pos.0 + 1;
            }
            // Falling down
            if falling_rock.pos.1 == 0
                || collision(&tunnel,
                             &RockPos {
                                 rref: falling_rock.rref,
                                 pos: (falling_rock.pos.0, falling_rock.pos.1 - 1),
                             }) {
                // found rock's resting spot
                break;
            }
            falling_rock.pos.1 -= 1;
        }
        tunnel.push(falling_rock);

        roc_index = (roc_index + 1) % 5;
        y_start = tunnel.iter().map(|p| p.pos.1 + p.rref.bbox.1.1 + 1)
            .max().unwrap_or(0);

        // dropping rocks that never would be reached
        tunnel = tunnel.into_iter().filter(|p| y_start - p.pos.1 < 100).collect();
    }
    //debug
    /*
    let points = tunnel.into_iter().flat_map(|p| p.rref.ps.iter()
            .map(|t| (t.0 + p.pos.0, t.1 + p.pos.1))
            .collect::<Vec<(u64, u64)>>())
        .collect::<Vec<(u64, u64)>>();
    //println!("{:?}", points);
    for y in (0..y_start).rev() {
        for x in 0..8 {
            if points.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    */

    println!("Part 1: {}", y_start);

    roc_index = 0;
    gas_index = 0;
    tunnel = Vec::new();
    y_start = 0;
    let mut aligned_rock_index = 0;
    let mut first_step_nr_rocks = 0;
    let mut first_step_height = 0;
    let mut repeating_step_nr_rocks = 0;
    let mut repeating_step_height = 0;
    let mut remaining_rocks = 0;
    let mut remaining_y_start = 0;
    let mut remaining_rocks_count = 0;
    let mut rest_height = 0;
    let mut alignment_found = false;
    let mut repeat_found = false;
    let mut remaining_found = false;
    for r in 1..5000 {
        y_start += 3;
        let rock = &rocs[roc_index];
        let mut falling_rock = RockPos {
            pos: (2, y_start),
            rref: rock,
        };

        loop {
            // Gas stream
            let current_gas = gas[gas_index];
            if !alignment_found && r > 1 && gas_index == 0 {
                println!("{} Gas index zero, rock index {}", r, roc_index);
                aligned_rock_index = roc_index;
                first_step_nr_rocks = r;
                alignment_found = true;
                println!("{} alignment, first step rocks {}", r, first_step_nr_rocks);
            }
            if alignment_found && !repeat_found
                && r > first_step_nr_rocks
                && gas_index == 0 && roc_index == aligned_rock_index {
                println!("{} repeat found!", r);
                repeat_found = true;
            }
            gas_index = (gas_index + 1) % gas.len();
            //println!("Gas index {}", gas_index);
            if current_gas < 0 && falling_rock.pos.0 > 0
                && !collision(&tunnel,
                              &RockPos {
                                  rref: falling_rock.rref,
                                  pos: (falling_rock.pos.0 - 1, falling_rock.pos.1),
                              }) {
                falling_rock.pos.0 = falling_rock.pos.0 - 1;
            } else if current_gas > 0 && falling_rock.pos.0 + falling_rock.rref.bbox.1.0 < 6
                && !collision(&tunnel,
                              &RockPos {
                                  rref: falling_rock.rref,
                                  pos: (falling_rock.pos.0 + 1, falling_rock.pos.1),
                              }) {
                falling_rock.pos.0 = falling_rock.pos.0 + 1;
            }
            // Falling down
            if falling_rock.pos.1 == 0
                || collision(&tunnel,
                             &RockPos {
                                 rref: falling_rock.rref,
                                 pos: (falling_rock.pos.0, falling_rock.pos.1 - 1),
                             }) {
                // found rock's resting spot
                //println!("Hit resting place {:?}", falling_rock);
                break;
            }
            falling_rock.pos.1 -= 1;
        }
        tunnel.push(falling_rock);
        roc_index = (roc_index + 1) % 5;
        y_start = tunnel.iter().map(|p| p.pos.1 + p.rref.bbox.1.1 + 1)
            .max().unwrap_or(0);

        if alignment_found && first_step_height == 0 {
            /*println!("{} gas index after alignment settled \n\
                \tgas index {}\n\taligned y {}\n\tfalling y {}\n\ty diff {}, \n\t ystart {}\n\tystart diff {}",
                     r, gas_index, aligned_y, falling_rock.pos.1, aligned_y - falling_rock.pos.1,
                     y_start, aligned_y - y_start);
             */
            first_step_height = y_start;
            println!("{} alignment, first step height {}", r, first_step_height);
        }
        if repeat_found && repeating_step_nr_rocks == 0 {
            repeating_step_nr_rocks = r - first_step_nr_rocks;
            repeating_step_height = y_start - first_step_height;
            remaining_rocks = (1000000000000u64 - first_step_nr_rocks) % repeating_step_nr_rocks;
            remaining_y_start = y_start;
            remaining_found = true;

            println!("{} repeat, nr rocks {}, height {}, remaining rocks {}, remaining y start {}", r,
                     repeating_step_nr_rocks,
                     repeating_step_height,
                     remaining_rocks,
                     remaining_y_start);
        }
        if remaining_found && rest_height == 0 {
            if remaining_rocks == remaining_rocks_count {
                rest_height = y_start - remaining_y_start;
                // All done here
                println!("All done thanks for playing! Rest height {}", rest_height);
                break;
            }
            remaining_rocks_count += 1;
        }
        // dropping rocks that never would be reached
        tunnel = tunnel.into_iter().filter(|p| y_start - p.pos.1 < 100).collect();
    }

    /*
        removing first section of 15 rocks that doesn't repeat,
        then dividing by the section that does repeat (every 35 rocks)
            with two alignments on rock index and gas index
        then times the repeating sections height,
        and adding the first non-repeating sections height
        and finally last but not least adding the remaining rocks heights
     */
    let test = (1000000000000u64 - 15) / (50 - 15) * /*y_start*/ (78 - 25) + 25;

    //let rest = (1000000000000u64 - 1699) % 1715;
    //let result = (1000000000000u64 - 1699) / (3414 - 1699) * /*y_start*/ (5328 - 2651) + 2651
    //    + /* height for last 31 (rest) rocks */ 58;
    println!("Part 2: test {}", test);

    let final_result = (1000000000000u64 - first_step_nr_rocks) / repeating_step_nr_rocks
        * repeating_step_height
        + first_step_height
        + rest_height;
    println!("Part 2: {}", final_result);
}

fn collision(rested_rocks: &Vec<RockPos>, falling_rock: &RockPos) -> bool {
    let x = falling_rock.pos.0;
    let y = falling_rock.pos.1;
    for rested_rock in rested_rocks.iter()
        .filter(|p| y <= p.pos.1 + p.rref.bbox.1.1) {
        let rx = rested_rock.pos.0;
        let ry = rested_rock.pos.1;
        // check by rock parts
        let points: HashSet<(u64, u64)> = falling_rock.rref.ps.iter()
            .map(|p| (p.0 + x, p.1 + y)).collect();
        //println!("  Falling rock points {:?}", points);
        let rested_points: HashSet<(u64, u64)> = rested_rock.rref.ps.iter()
            .map(|p| (p.0 + rx, p.1 + ry)).collect();
        //println!("  Rested rock points {:?}", rested_points);
        let intersection = points.intersection(&rested_points).collect::<Vec<&(u64, u64)>>();
        if intersection.len() > 0 {
            //println!("  Collision: intersection points {:?}, pos {:?}, with rock {:?}", intersection, falling_rock.pos, rested_rock);
            return true;
        }
    }
    return false;
}

#[derive(Debug)]
struct Rock {
    bbox: ((u64, u64), (u64, u64)),
    ps: Vec<(u64, u64)>,
}

#[derive(Debug, Clone, Copy)]
struct RockPos<'a> {
    rref: &'a Rock,
    pos: (u64, u64),
}
