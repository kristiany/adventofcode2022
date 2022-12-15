use std::collections::HashSet;
use std::ops::Sub;
use std::time::Instant;

fn main() {
    let input = include_str!("../inputs/fifteen/input.in");
    let sensors = input.split("\n").into_iter()
        .map(| line | Sensor::parse(line))
        .collect::<Vec<Sensor>>();

    let start1 = Instant::now();
    let min_x: i32 = sensors.iter().map(|b | b.sensor.0 - b.dist as i32).min().unwrap();
    let max_x: i32 = sensors.iter().map(|b | b.sensor.0 + b.dist as i32).max().unwrap();
    println!("min x {}, max x {}", min_x, max_x);
    println!("sensors: {:?}", sensors);
    let beacons = sensors.iter().map(| b | b.beacon).collect::<HashSet<Pos>>();
    //let y = 10; // test
    let y = 2000000;

    let mut no_beacon = 0;
    for x in min_x..=max_x {
        for sens in &sensors {
            if !beacons.contains(&(x, y)) && sens.in_sensor_radius(&(x, y)) {
                //println!(" in radius {:?}", &(x, y));
                no_beacon += 1;
                break;
            }
        }
    }

    println!("Time: {}", start1.elapsed().as_secs());
    println!("Part 1: {}", no_beacon);

    let start2 = Instant::now();
    let mut distress_beacon: Option<Pos> = None;
    'outer: for sens in &sensors {
        'pos: for pos in to_edge_positions(&(sens.sensor.0, sens.sensor.1 - sens.dist as i32 - 1), sens.sensor) {
            for other in sensors.iter().filter(| s | *s != sens) {
                if other.in_sensor_radius(&pos) {
                    continue 'pos;
                }
            }
            println!(" out of radius {:?}", &pos);
            distress_beacon = Some(pos);
            break 'outer;

        }
    }

    assert!(distress_beacon != None);
    println!("Time: {}", start2.elapsed().as_secs());
    println!("Part 2: {:?} = {}", distress_beacon.unwrap(),
             distress_beacon.unwrap().0 as u128 * 4000000 + distress_beacon.unwrap().1 as u128);
}

fn to_edge_positions(p: &Pos, sensor: Pos) -> HashSet<Pos> {
    //println!("p: {:?}, sensor: {:?}", p, sensor);
    let mut result: Vec<Pos> = Vec::new();
    let edge_len = (sensor.1 - p.1).abs();
    for z in 0..=edge_len {
        result.push((p.0 + z, p.1 + z));
    }
    for z in 0..=edge_len {
        result.push((p.0 - z, p.1 + z));
    }
    let bp = (p.0, p.1 + edge_len * 2);
    for z in 0..=edge_len {
        result.push((bp.0 + z, bp.1 - z));
    }
    for z in 0..=edge_len {
        result.push((bp.0 - z, bp.1 - z));
    }
    //println!("    edge: {:?}", result);
    result.into_iter().filter(| p | p.0 >= 0 && p.0 <= 4000000 && p.1 >= 0 && p.1 <= 4000000)
        .collect::<HashSet<Pos>>()
}

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Clone, Debug)]
struct Sensor {
    beacon: Pos,
    sensor: Pos,
    dist: usize
}

impl Sensor {
    fn parse(input: &str) -> Sensor {
        let sens_input = input.split(":").into_iter().collect::<Vec<&str>>()[0]
            .split(",").into_iter().collect::<Vec<&str>>();
        let sens_x = sens_input[0].split("=").into_iter()
            .collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let sens_y = sens_input[1].split("=").into_iter()
            .collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        let beac_input = input.split(":").into_iter().collect::<Vec<&str>>()[1]
            .split(",").into_iter().collect::<Vec<&str>>();
        let beac_x = beac_input[0].split("=").into_iter()
            .collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let beac_y = beac_input[1].split("=").into_iter()
            .collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let dist = ((beac_x - sens_x).abs() + (beac_y - sens_y).abs()) as usize;
        Sensor { beacon: (beac_x, beac_y), sensor: (sens_x, sens_y), dist: dist }
    }

    fn sensor_dist(&self, p : &Pos) -> usize {
        ((self.sensor.0 - p.0).abs() + (self.sensor.1 - p.1).abs()) as usize
    }

    fn in_sensor_radius(&self, p: &Pos) -> bool {
        let dist = self.sensor_dist(p);
        dist <= self.dist
    }
}