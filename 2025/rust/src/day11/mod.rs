use std::collections::HashMap;

const YOU: [u8; 3] = *b"you";
const OUT: [u8; 3] = *b"out";
const SVR: [u8; 3] = *b"svr";
const DAC: [u8; 3] = *b"dac";
const FFT: [u8; 3] = *b"fft";

// All Device id's are 3 bytes long!
type Node = [u8; 3];

fn parse_node(node: &str) -> Node {
    node.as_bytes().try_into().unwrap()
}

fn parse_graph(input: &str) -> HashMap<Node, Vec<Node>> {
    input
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(": ").unwrap();
            (
                parse_node(source),
                destinations.split_whitespace().map(parse_node).collect(),
            )
        })
        .collect()
}

fn count_paths(graph: &HashMap<Node, Vec<Node>>, current: Node, end: Node) -> usize {
    if current == end {
        return 1;
    }
    graph
        .get(&current)
        .unwrap()
        .iter()
        .map(|&neighbour| count_paths(graph, neighbour, end))
        .sum()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 607
    let graph = parse_graph(input);
    count_paths(&graph, YOU, OUT)
}

fn count_paths_with_dac_fft(
    graph: &HashMap<Node, Vec<Node>>,
    cache: &mut HashMap<(Node, bool, bool), usize>,
    current: Node,
    end: Node,
    seen_dac: bool,
    seen_fft: bool,
) -> usize {
    if current == end {
        return usize::from(seen_dac && seen_fft);
    }
    let cache_key = (current, seen_dac, seen_fft);
    if let Some(&cached_total) = cache.get(&cache_key) {
        return cached_total;
    }
    let total = graph
        .get(&current)
        .unwrap()
        .iter()
        .map(|&neighbour| {
            count_paths_with_dac_fft(
                graph,
                cache,
                neighbour,
                end,
                seen_dac || current == DAC,
                seen_fft || current == FFT,
            )
        })
        .sum();
    cache.insert(cache_key, total);
    total
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 506264456238938
    let graph = parse_graph(input);
    let mut cache: HashMap<(Node, bool, bool), usize> = HashMap::new();
    count_paths_with_dac_fft(&graph, &mut cache, SVR, OUT, false, false)
    // TODO: can split problem into 3 times part1, but then there are paths that can "go past" your target node
    // FIXME: validate assumption that FFT -> DAC and not other way around
    // let p1 = count_paths(&graph, SVR, FFT);
    // let p2 = count_paths(&graph, FFT, DAC);
    // let p3 = count_paths(&graph, DAC, OUT);
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
