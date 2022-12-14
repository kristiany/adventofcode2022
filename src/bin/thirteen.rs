use std::iter::Peekable;

fn main() {
    let input = include_str!("../inputs/thirteen/input.in");
    let pairs : Vec<&str> = input.split("\n").into_iter().filter(| r | ! r.is_empty())
        .collect();
    let mut result : Vec<usize> = Vec::new();
    for i in (0..pairs.len()).step_by(2) {
        let left = pairs[i];
        let right = pairs[i + 1];
        println!("Comparing {}", left);
        println!("      and {}", right);
        let a = Node::parse(left);
        let b = Node::parse(right);
        if a < b {
            println!("Result: {} is ordered", i / 2 + 1);
            result.push(i / 2 + 1);
        }
        else {
            println!("Result: {} is just wrong", i / 2 + 1);
        }
    }
    println!("{:?}", result);
    println!("Part 1: {}", result.iter().sum::<usize>());

    let mut nodes = pairs.iter().map(| l | Node::parse(l).unwrap()).collect::<Vec<Node>>();
    let div_2 = Node::parse("[[2]]").unwrap();
    let div_6 = Node::parse("[[6]]").unwrap();
    nodes.push(div_2.clone());
    nodes.push(div_6.clone());
    nodes.sort();
    /*for n in &nodes {
        println!("{:?}", n);
    }*/
    let index_2 = nodes.iter().enumerate().find(| n | *n.1 == div_2).unwrap().0 + 1;
    let index_6 = nodes.iter().enumerate().find(| n | *n.1 == div_6).unwrap().0 + 1;
    println!("Part 2: {}", index_2 * index_6);
}

// This is more or less stolen from https://github.com/quat1024/hatchery/blob/trunk/advent2022/src/day13.rs
#[derive(PartialEq, Eq, Clone, Debug)]
enum Node {
    Nr(usize),
    List(Vec<Node>),
}

impl Node {
    fn parse(line: &str) -> Option<Node> {
        Self::parse_rec(&mut line.chars().peekable())
    }

    fn parse_rec(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Node> {
        match chars.next() {
            Some(c) if c.is_ascii_digit() => {
                // Only two digit number is 10
                if let Some('0') = chars.peek() {
                    chars.next();
                    Some(Node::Nr(10))
                } else {
                    Some(Node::Nr(c.to_digit(10).unwrap() as usize))
                }
            },
            Some('[') => {
                let mut nodes = Vec::new();
                while let Some(node) = Self::parse_rec(chars) {
                    nodes.push(node);
                    if Some(']') == chars.next() {
                        break;
                    }
                }
                Some(Node::List(nodes))
            },
            _ => None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, right: &Self) -> std::cmp::Ordering {
        match (self, right) {
            (Node::Nr(left_nr), Node::Nr(right_nr)) => left_nr.cmp(right_nr),
            (Node::List(left_list), Node::List(right_list)) => left_list
                .iter()
                .zip(right_list.iter())
                .find_map(| (left_node, right_node) | {
                    let result = left_node.cmp(right_node);
                    result.is_ne().then_some(result)
                })
                .unwrap_or_else(|| left_list.len().cmp(&right_list.len())),
            (Node::Nr(_), Node::List(_)) => Node::List(vec![self.clone()]).cmp(right),
            (Node::List(_), Node::Nr(_)) => self.cmp(&Node::List(vec![right.clone()])),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
