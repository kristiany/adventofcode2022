use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/eighteen/input.in");

    let lava_list = input.split("\n").into_iter().map(|r| {
        let row = r.split(",").collect::<Vec<&str>>();
        return P{
            x: row[0].parse::<i32>().unwrap(),
            y: row[1].parse::<i32>().unwrap(),
            z: row[2].parse::<i32>().unwrap()
        };
    }).collect::<Vec<P>>();
    //println!("{:?}", lava_list);

    let lavas: HashSet<P> = HashSet::from_iter(lava_list.iter().cloned());
    let mut surface_area = 0;
    for drop_i in 0..lava_list.len() {
        let drop = lava_list[drop_i];
        surface_area += get_adjacent_air(&lavas, &drop).len();
    }
    println!("Part 1: {}", surface_area);

    let min_x = lava_list.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap();
    let max_x = lava_list.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap();
    let min_y = lava_list.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap();
    let max_y = lava_list.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap();
    let min_z = lava_list.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap();
    let max_z = lava_list.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap();

    println!("Min x {:?}, max x {:?}", min_x, max_x);
    println!("Min y {:?}, max y {:?}", min_y, max_y);
    println!("Min z {:?}, max z {:?}", min_z, max_z);

    /*
        1. fill bounding box with "outer air" around the lava
        2. count all surfaces that touches the lava
    */
    let mut outer_surface_area = 0;
    let max_x_air = P { x: max_x.x + 1, y: max_x.y, z: max_x.z };
    let mut outer_air = HashSet::<P>::new();
    let mut to_visit = Vec::<P>::new();
    to_visit.push(max_x_air);

    while to_visit.len() > 0 {
        //println!("to visit {:?}", to_visit);
        let n = to_visit.pop().unwrap();
        if outer_air.contains(&n) {
            continue;
        }
        outer_air.insert(n);
        outer_surface_area += get_adjacent_lava(&lavas, &n).len();

        let airs = get_adjacent_air(&lavas, &n);
        airs.into_iter()
            .filter(|p| ! outer_air.contains(p))
            .filter(|p| p.x >= min_x.x - 1 && p.x <= max_x.x + 1
                && p.y >= min_y.y - 1 && p.y <= max_y.y + 1
                && p.z >= min_z.z - 1 && p.z <= max_z.z + 1)
            .for_each(|p| to_visit.push(p));
    }

    println!("Part 2: {}", outer_surface_area);
}

fn get_adjacent_air(lavas: &HashSet<P>, n: &P) -> Vec<P> {
    let mut points = Vec::<P>::new();
    points.push(P { x: n.x - 1, y: n.y, z: n.z });
    points.push(P { x: n.x + 1, y: n.y, z: n.z });
    points.push(P { x: n.x, y: n.y - 1, z: n.z });
    points.push(P { x: n.x, y: n.y + 1, z: n.z });
    points.push(P { x: n.x, y: n.y, z: n.z - 1 });
    points.push(P { x: n.x, y: n.y, z: n.z + 1 });
    return points.into_iter()
        .filter(|p| ! lavas.contains(p))
        .collect();
}

fn get_adjacent_lava(lavas: &HashSet<P>, n: &P) -> Vec<P> {
    let mut points = Vec::<P>::new();
    points.push(P { x: n.x - 1, y: n.y, z: n.z });
    points.push(P { x: n.x + 1, y: n.y, z: n.z });
    points.push(P { x: n.x, y: n.y - 1, z: n.z });
    points.push(P { x: n.x, y: n.y + 1, z: n.z });
    points.push(P { x: n.x, y: n.y, z: n.z - 1 });
    points.push(P { x: n.x, y: n.y, z: n.z + 1 });
    return points.into_iter()
        .filter(|p| lavas.contains(p))
        .collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct P {
    x: i32,
    y: i32,
    z: i32,
}