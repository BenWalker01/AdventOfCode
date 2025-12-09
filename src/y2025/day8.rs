pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

pub struct JnctBox {
    x: u32,
    y: u32,
    z: u32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut current = x;
        while self.parent[current] != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }
        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }
}

fn get_sorted_edges(boxes: &[JnctBox]) -> Vec<(u32, usize, usize)> {
    let mut edges: Vec<(u32, usize, usize)> = Vec::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let dx = boxes[i].x as i64 - boxes[j].x as i64;
            let dy = boxes[i].y as i64 - boxes[j].y as i64;
            let dz = boxes[i].z as i64 - boxes[j].z as i64;
            let distance_sq = (dx * dx + dy * dy + dz * dz) as u64;
            edges.push(((distance_sq as f64).sqrt() as u32, i, j));
        }
    }
    edges.sort_by_key(|e| e.0);
    edges
}

fn closest_pair(boxes: &[JnctBox]) -> Option<Vec<(usize, usize, u32)>> {
    if boxes.len() < 2 {
        return None;
    }

    let edges = get_sorted_edges(boxes);
    let pairs: Vec<(usize, usize, u32)> = edges
        .iter()
        .take(1000)
        .map(|(dist, i, j)| (*i, *j, *dist))
        .collect();

    if pairs.is_empty() {
        None
    } else {
        Some(pairs)
    }
}

fn count_networks_after_n_connections(boxes: &[JnctBox], n: usize) -> (usize, Vec<usize>) {
    let edges = get_sorted_edges(boxes);
    let mut uf = UnionFind::new(boxes.len());

    for (_, idx1, idx2) in edges.iter().take(n) {
        uf.union(*idx1, *idx2);
    }

    let mut circuits: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for i in 0..boxes.len() {
        let root = uf.find(i);
        *circuits.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = circuits.values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    (sizes.len(), sizes)
}

fn part1(input: &str) -> String {
    let data: Vec<&str> = input.trim().lines().collect();
    let mut boxes: Vec<JnctBox> = Vec::new();
    for line in data {
        let line: Vec<&str> = line.split(",").collect();
        let x = line[0].parse::<u32>().unwrap();
        let y = line[1].parse::<u32>().unwrap();
        let z = line[2].parse::<u32>().unwrap();
        boxes.push(JnctBox { x, y, z });
    }

    if let Some(pairs) = closest_pair(&boxes) {
        for (i, (idx1, idx2, dist)) in pairs.iter().enumerate() {
            let box1 = &boxes[*idx1];
            let box2 = &boxes[*idx2];
            println!(
                "Pair {}: ({},{},{}) <-> ({},{},{}) - Distance: {}",
                i + 1,
                box1.x,
                box1.y,
                box1.z,
                box2.x,
                box2.y,
                box2.z,
                dist
            );
        }

        let mut uf = UnionFind::new(boxes.len());
        for (idx1, idx2, _) in &pairs {
            uf.union(*idx1, *idx2);
        }

        let mut circuits: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for i in 0..boxes.len() {
            let root = uf.find(i);
            circuits.entry(root).or_default().push(i);
        }

        println!("\nCircuits after 1000 connections:");
        let mut sizes: Vec<(usize, Vec<usize>)> = circuits
            .into_values()
            .map(|boxes| (boxes.len(), boxes))
            .collect();
        sizes.sort_by(|a, b| b.0.cmp(&a.0));

        for (i, (size, box_indices)) in sizes.iter().enumerate() {
            println!("Circuit {}: {} boxes {:?}", i + 1, size, box_indices);
        }

        if sizes.len() >= 3 {
            let product = sizes[0].0 * sizes[1].0 * sizes[2].0;
            println!(
                "\nThree largest circuits: {} x {} x {} = {}",
                sizes[0].0, sizes[1].0, sizes[2].0, product
            );
            return product.to_string();
        }
    }

    "ERROR".to_string()
}

fn part2(input: &str) -> String {
    let data: Vec<&str> = input.trim().lines().collect();
    let mut boxes: Vec<JnctBox> = Vec::new();
    for line in data {
        let line: Vec<&str> = line.split(",").collect();
        let x = line[0].parse::<u32>().unwrap();
        let y = line[1].parse::<u32>().unwrap();
        let z = line[2].parse::<u32>().unwrap();
        boxes.push(JnctBox { x, y, z });
    }

    let edges = get_sorted_edges(&boxes);
    let mut left = 0;
    let mut right = edges.len();
    let mut step = 0;

    while left < right {
        let mid = (left + right) / 2;
        let (num_networks, _) = count_networks_after_n_connections(&boxes, mid);
        println!("Checking step {}: {} networks", mid, num_networks);

        if num_networks > 1 {
            left = mid + 1;
        } else {
            step = mid;
            right = mid;
        }
    }

    let edges = get_sorted_edges(&boxes);
    if step > 0 && step <= edges.len() {
        let (dist, idx1, idx2) = edges[step - 1];
        let box1 = &boxes[idx1];
        let box2 = &boxes[idx2];
        println!(
            "Last connection: ({},{},{}) <-> ({},{},{}) - Distance: {}",
            box1.x, box1.y, box1.z, box2.x, box2.y, box2.z, dist
        );
        return (box1.x as u64 * box2.x as u64).to_string();
    }

    "ERROR".to_string()
}
