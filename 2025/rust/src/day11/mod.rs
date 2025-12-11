use std::collections::HashMap;

// All Device id's are 3 bytes long!

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default)]
// struct Node([u8; 3]);
// impl Deref for Node {
//     type Target = [u8; 3];
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl From<[u8; 3]> for Node {
//     fn from(value: [u8; 3]) -> Self {
//         Self(value)
//     }
// }
// fn parse_node(node: &str) -> Node {
//     let bytes: [u8; 3] = node.as_bytes().try_into().unwrap();
//     bytes.into()
//     // node.as_bytes().try_into().unwrap().into()
// }

// Maybe a simple type alias is better...
type Node = [u8; 3];

// const fn parse_node(node: &str) -> Node {
//     // node.as_bytes().try_into().unwrap()
//     let b = node.as_bytes();
//     assert!(b.len() == 3, "node must be a 3 byte string");
//     [b[0], b[1], b[2]]
// }

fn count_paths(current: Node, end: Node, graph: &HashMap<Node, Vec<Node>>) -> usize {
    if current == end {
        return 1;
    }

    let mut total = 0usize;
    // if let Some(neighbours) = graph.get(&current) {
    //     for &v in neighbours {
    //         total += count_paths(v, end, graph);
    //     }
    // }
    let neighbours = graph.get(&current).unwrap();
    for &n in neighbours {
        total += count_paths(n, end, graph);
    }
    total
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = ?

    const START: [u8; 3] = *b"you";
    const END: [u8; 3] = *b"out";

    let graph: HashMap<Node, Vec<Node>> = input
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(": ").unwrap();
            (
                // parse_node(source),
                source.as_bytes().try_into().unwrap(),
                destinations
                    .split_whitespace()
                    // .map(parse_node)
                    .map(|d| d.as_bytes().try_into().unwrap())
                    .collect(),
            )
        })
        .collect();

    count_paths(START, END, &graph)
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    input.lines().count();
    0
}

#[cfg(test)]
const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

#[test]
fn part1_examples() {
    assert_eq!(solve_part1(EXAMPLE), 5);
}

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE), 0);
}
