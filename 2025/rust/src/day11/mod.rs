// /// Node
// struct Device {
//     id: &str,
// }

// /// Directed Edge
// struct Connection {
//     from: &str,
//     to: &str,
// }

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = ?
    // input.lines().count();

    let mut devices: Vec<&str> = vec![];
    let mut connections: Vec<(&str, &str)> = vec![];
    // TODO: make this a Map?

    for line in input.lines() {
        if let Some((device, rest)) = line.split_once(": ") {
            devices.push(device);
            connections.extend(rest.split_ascii_whitespace().map(|dest| (device, dest)));
        }
    }

    0
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    input.lines().count();
    0
}

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
fn part1_examples() {}

#[test]
fn part2_examples() {}
