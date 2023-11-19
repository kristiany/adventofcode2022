#![feature(pattern)]

use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
use regex::Regex;
use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/sixteen/input.in");

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut valves: Vec<Valve> = Vec::new();
    for line in input.split("\n").into_iter() {
        let name = line.split(" ").into_iter().collect::<Vec<&str>>()[1];
        let rate = line.split(";").into_iter().collect::<Vec<&str>>()[0]
            .split("=").into_iter().collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        valves.push(Valve { name: name, flow_rate: rate });

        let downstream = line.split(&Regex::new(r"valve(s)? ").unwrap())
            .into_iter().collect::<Vec<&str>>()[1]
            .split(", ").into_iter().collect::<Vec<&str>>();
        graph.insert(name, downstream);
    }

    let significant_nodes : HashMap<&str, usize> = valves.iter()
        .filter(| n | n.flow_rate > 0)
        .map(| n | (n.name.clone(), n.flow_rate))
        .into_iter().collect::<HashMap<&str, usize>>();

    // Check for each initial path start
    let mut max_pressure = 0usize;
    let mut init_paths: Vec<((&str, &str), Vec<String>)> = Vec::new();
    for n in significant_nodes.keys().into_iter().clone()
            .map(| n | *n)
            .collect::<HashSet<&str>>() {
        init_paths.push((("AA", n), astar("AA", &n.to_owned(), &graph, &significant_nodes).unwrap()));
    }
    for init_path in init_paths {
        let mut closed_valves = significant_nodes.keys().into_iter().clone()
            .map(| n | *n)
            .collect::<HashSet<&str>>();
        let mut timeline: HashMap<usize, usize> = HashMap::new();
        let mut minute: usize = 0;
        let mut cur = init_path.0.1.clone();
        closed_valves.remove(&cur);
        minute += 1;
        minute += &init_path.1.len();
        if minute > 30 { break; }
        println!("Init Min {}, open {}, rate {}", minute, &cur, significant_nodes[cur]);
        timeline.insert(minute, significant_nodes[cur]);

        while &closed_valves.len() > &0 {
            let mut paths: Vec<((&str, &str), Vec<String>)> = Vec::new();
            for n in &closed_valves {
                let path = astar(&cur, &n.to_owned(), &graph, &significant_nodes);
                paths.push(((cur, n), path.unwrap()));
            }
            //println!("{:?}", paths);
            paths.sort_by_key(| e |
                std::cmp::max(30i32 - minute as i32 - e.1.len() as i32 - 1, 0) as usize * significant_nodes[&e.0.1]);
            for p in &paths {
                println!("    {:?} len {}, rate {}, sort score {}", p.0, p.1.len(), significant_nodes[&p.0.1],
                     std::cmp::max(30i32 - minute as i32 - p.1.len() as i32 - 1, 0) as usize * significant_nodes[&p.0.1]);
            }
            let best = paths.pop().unwrap();
            //println!("Route: {:?}", best);
            cur = &best.0.1.clone();
            closed_valves.remove(&cur);
            minute += 1;
            minute += &best.1.len();
            if minute > 30 { break; }
            println!("Min {}, open {}, rate {}", minute, &cur, significant_nodes[cur]);
            timeline.insert(minute, significant_nodes[cur]);
        }
        println!("min {}, {:?}", minute, timeline);
        let pressure : usize = timeline.iter()
            .map(| e | (30 - std::cmp::min(e.0, &30usize)) * e.1).sum();
        println!("    Pressure: {}, max: {}", pressure, max_pressure);
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }

    println!("Part 1: {}", max_pressure);

    let result2 = part_2(input);
    println!("Part 2: {}", result2);

}

#[derive(Clone, Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
fn astar<'a>(from: &'a str, to: &'a str, graph: &'a HashMap<&'a str, Vec<&'a str>>, rates: &'a HashMap<&str, usize>) -> Option<Vec<String>> {
    // A* \o/
    let mut queue : Vec<&str> = Vec::new();
    queue.push(from);
    let mut path: HashMap<&str, &str> = HashMap::new();
    let mut gscore: HashMap<&str, usize> = HashMap::new();
    gscore.insert(from, 0);
    let mut fscore: HashMap<&str, usize> = HashMap::new();
    fscore.insert(from, heur(*rates.get(&to).unwrap_or(&1usize), 0));

    while let Some(cur) = queue.pop() {
        if cur == to {
            let found = backtrack(&path, &cur);
            return Some(found);
        }

        for next in &graph[cur] {
            //println!("      Cur {:?}, next {:?}", map[cur.1][cur.0], map[p.1][p.0]);
            let tentative_gscore = gscore[&cur] + 1;
            if tentative_gscore < *gscore.get(next).unwrap_or(&usize::MAX) {
                path.insert(next, cur);
                gscore.insert(next, tentative_gscore);
                //fscore.insert(next, tentative_gscore + rates[&to]);
                fscore.insert(next, heur(*rates.get(&to).unwrap_or(&1usize), tentative_gscore));
                if !queue.contains(&next) {
                    //insert_sorted(&next, &mut queue, &fscore);
                    queue.push(&next);
                    queue.sort();
                }
            }
        }
    }
    None
}

fn heur(rate : usize, steps : usize) -> usize {
    (30 - steps) * rate
}

fn backtrack<'a>(path : &'a HashMap<&'a str, &'a str>, cur: &'a str) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    let mut next = cur;
    result.push(cur.to_string());
    while path.contains_key(&next) {
        next = &path[&next];
        result.push(next.to_string());
    }
    result.pop(); // Remove current
    result.reverse();
    result
}

// Part 2 is not my solution, I gave up :(
// https://nickymeuleman.netlify.app/garden/aoc2022-day16
pub fn part_2(input: &str) -> u32 {
    let map = parse(input);
    let dist_map = min_distances(&map); // key: (from, to), value: move_cost
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    // key: opened, val: relieved_at_end
    let mut max_relieved_states: HashMap<BTreeSet<&str>, u32> = HashMap::new();

    let mut q = VecDeque::new();
    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    while let Some(State {
                       opened,
                       curr,
                       elapsed,
                       relieved,
                   }) = q.pop_front()
    {
        let relieved_at_end = wait_until_ending(26, elapsed, relieved, &opened, &map);
        // record state. only update state if it beats the `relieved_at_end` number
        max_relieved_states
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        // if all flowing valves are opened or the timelimit was reached, skip
        if opened.len() == flowing.len() || elapsed >= 26 {
            continue;
        }
        // for every unopened valve, run simulation
        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            // how long would moving to dest take? +1 to open the valve
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            // if opening the dest valve would exceed the time limit, skip
            if new_elapsed >= 26 {
                continue;
            }

            // relieve pressure of opened valves while we move to dest and open it
            let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + (relieved_per_min * cost);

            // add opened valve to opened valves
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            q.push_back(State {
                opened: new_opened,
                curr: dest,
                elapsed: new_elapsed,
                relieved: new_relieved,
            });
        }
    }

    max_relieved_states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap()
}

fn wait_until_ending(
    max_time: u32,
    elapsed: u32,
    relieved: u32,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve2>,
) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
    relieved + (relieved_per_min * time_left)
}

/// map shortest distance from "AA" to any flowing valve
/// map shortest distance from any flowing valve to an other
fn min_distances<'a>(map: &'a HashMap<&str, Valve2>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        // only keep flowing valves
        .filter(|(_, valve)| valve.flow > 0)
        // get the name of flowing valves
        .map(|(&name, _)| name)
        // iterate over every combination of 2 flowing valves
        .tuple_combinations()
        // record shortest distance between those 2
        // (and the dist from "AA" to a flowing valve because we start there)
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            // from AA to name1
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            // from AA to name2
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));

            let dist = min_cost(name1, name2, map);
            // from name1 to name2
            acc.insert((name1, name2), dist);
            // from name2 to name1
            acc.insert((name2, name1), dist);

            acc
        })
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// return lowest cost to move from a valve to an other valve
fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve2>) -> u32 {
    // shortest path:
    // Dijkstra's algorithm
    // nodes in the priority queue are sorted so the lowest cost gets popped first
    let mut pq = BinaryHeap::new();
    // prevent backtracking by keeping track of valve rooms we already saw
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: 0,
        curr: from,
    });
    seen.insert(from);

    while let Some(Node { cost, curr }) = pq.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].neighbours.iter() {
            // only insert into the pq if we did not already see the neighbour valve
            if seen.insert(neighbour) {
                pq.push(Node {
                    cost: cost + 1,
                    curr: neighbour,
                });
            }
        }
    }

    u32::MAX
}

struct Valve2<'a> {
    flow: u32,
    neighbours: HashSet<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Valve2> {
    input
        .lines()
        .map(|line| {
            let (valve, neighbours) = line.split_once("; ").unwrap();
            let valve = valve.strip_prefix("Valve ").unwrap();
            let (name, flow) = valve.split_once(" has flow rate=").unwrap();
            let flow = flow.parse().unwrap();
            let neighbours = neighbours
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| neighbours.strip_prefix("tunnel leads to valve "))
                .unwrap();
            let neighbours = neighbours.split_terminator(", ").collect();

            (name, Valve2 { flow, neighbours })
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: u32,
    relieved: u32,
}
