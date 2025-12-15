use std::collections::HashMap;

const YOU: [u8; 3] = *b"you";
const OUT: [u8; 3] = *b"out";
const SVR: [u8; 3] = *b"svr";
const DAC: [u8; 3] = *b"dac";
const FFT: [u8; 3] = *b"fft";

// All Device id's are 3 bytes long!
type Node = [u8; 3];

fn parse_node(node: &str) -> Node {
    let b = node.as_bytes();
    assert!(b.len() == 3, "node id must be 3 byte length");
    [b[0], b[1], b[2]]
}

fn parse_graph(input: &str) -> HashMap<Node, Vec<Node>> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").map(|(source, destinations)| {
                (
                    parse_node(source),
                    destinations.split_whitespace().map(parse_node).collect(),
                )
            })
        })
        .collect()
}

fn count_paths(
    graph: &HashMap<Node, Vec<Node>>,
    cache: &mut HashMap<(Node, bool, bool), usize>,
    current: Node,
    target: Node,
    seen_dac: bool,
    seen_fft: bool,
) -> usize {
    if current == target {
        return usize::from(seen_dac && seen_fft);
    }
    let cache_key = (current, seen_dac, seen_fft);
    if let Some(&cached_total) = cache.get(&cache_key) {
        return cached_total;
    }
    let Some(neighbours) = graph.get(&current) else {
        return 0;
    };
    let total = neighbours
        .iter()
        .map(|&neighbour| {
            count_paths(
                graph,
                cache,
                neighbour,
                target,
                seen_dac || current == DAC,
                seen_fft || current == FFT,
            )
        })
        .sum();
    cache.insert(cache_key, total);
    total
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 607
    let graph = parse_graph(input);
    let mut cache: HashMap<(Node, bool, bool), usize> = HashMap::new();
    count_paths(&graph, &mut cache, YOU, OUT, true, true)
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 506264456238938
    let graph = parse_graph(input);
    let mut cache: HashMap<(Node, bool, bool), usize> = HashMap::new();
    count_paths(&graph, &mut cache, SVR, OUT, false, false)

    // This works, but is not faster

    // let p1 = count_paths(&graph, &mut cache, SVR, FFT, true, true);
    // cache.clear();
    // let p2 = count_paths(&graph, &mut cache, FFT, DAC, true, true);
    // cache.clear();
    // let p3 = count_paths(&graph, &mut cache, DAC, OUT, true, true);
    // p1 * p2 * p3
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

#[cfg(test)]
const EXAMPLE_PART2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE_PART2), 2);
}
