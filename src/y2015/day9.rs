use petgraph::graph::UnGraph;
use std::collections::HashMap;

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
fn part1(input: &str) -> String {
    let res = generate_graph(input);
    let g = res.0;
    let nodes = res.1;

    let node_list: Vec<petgraph::graph::NodeIndex> = nodes.values().cloned().collect();
    let n = node_list.len();
    let mut best = i32::MAX;

    let mut used = vec![false; n];
    let mut order: Vec<petgraph::graph::NodeIndex> = Vec::with_capacity(n);
    backtrack(&g, &node_list, &mut used, &mut order, &mut best);

    best.to_string()
}

fn generate_graph(
    input: &str,
) -> (
    petgraph::Graph<&str, i32, petgraph::Undirected>,
    std::collections::HashMap<&str, petgraph::prelude::NodeIndex>,
) {
    let mut g = UnGraph::<&str, i32>::new_undirected();
    let mut nodes: HashMap<&str, petgraph::graph::NodeIndex> = HashMap::new();

    for line in input.trim().lines() {
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap();
        let _to_word = parts.next();
        let to = parts.next().unwrap();
        let _eq = parts.next();
        let dist = parts.next().unwrap().parse::<i32>().unwrap();

        let a = if let Some(&idx) = nodes.get(from) {
            idx
        } else {
            let idx = g.add_node(from);
            nodes.insert(from, idx);
            idx
        };

        let b = if let Some(&idx) = nodes.get(to) {
            idx
        } else {
            let idx = g.add_node(to);
            nodes.insert(to, idx);
            idx
        };

        g.add_edge(a, b, dist);
    }
    (g, nodes)
}

fn backtrack(
    g: &UnGraph<&str, i32>,
    node_list: &Vec<petgraph::graph::NodeIndex>,
    used: &mut Vec<bool>,
    order: &mut Vec<petgraph::graph::NodeIndex>,
    best: &mut i32,
) {
    if order.len() == node_list.len() {
        let mut cost = 0;
        for w in order.windows(2) {
            let a = w[0];
            let b = w[1];
            if let Some(e) = g.find_edge(a, b) {
                cost += *g.edge_weight(e).unwrap();
            } else {
                return;
            }
        }
        if cost < *best {
            *best = cost;
        }
        return;
    }

    for i in 0..node_list.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        order.push(node_list[i]);
        backtrack(g, node_list, used, order, best);
        order.pop();
        used[i] = false;
    }
}

fn backtrack_but_more(
    g: &UnGraph<&str, i32>,
    node_list: &Vec<petgraph::graph::NodeIndex>,
    used: &mut Vec<bool>,
    order: &mut Vec<petgraph::graph::NodeIndex>,
    best: &mut i32,
) {
    if order.len() == node_list.len() {
        let mut cost = 0;
        for w in order.windows(2) {
            let a = w[0];
            let b = w[1];
            if let Some(e) = g.find_edge(a, b) {
                cost += *g.edge_weight(e).unwrap();
            } else {
                return;
            }
        }
        if cost > *best {
            *best = cost;
        }
        return;
    }

    for i in 0..node_list.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        order.push(node_list[i]);
        backtrack_but_more(g, node_list, used, order, best);
        order.pop();
        used[i] = false;
    }
}

fn part2(input: &str) -> String {
    let res = generate_graph(input);
    let g = res.0;
    let nodes = res.1;

    let node_list: Vec<petgraph::graph::NodeIndex> = nodes.values().cloned().collect();
    let n = node_list.len();
    let mut best = i32::MIN;

    let mut used = vec![false; n];
    let mut order: Vec<petgraph::graph::NodeIndex> = Vec::with_capacity(n);
    backtrack_but_more(&g, &node_list, &mut used, &mut order, &mut best);

    best.to_string()
}
