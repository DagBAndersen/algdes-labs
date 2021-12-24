use priority_queue::PriorityQueue;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
};

#[derive(Debug)]
struct Node {
    index: usize,
    weight: usize,
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn main() {
    // get string from args
    let args: Vec<String> = std::env::args().collect();
    if let Some(file) = &args.get(1) {
        println!("{}", file);

        let graph = parse_input(&file);

        dbg!(&graph.start);
        dbg!(&graph.end);
        dbg!(&graph.nodes);

        dbg!(&graph.edges);

        let path = lowest_weight(&graph);

        println!("path: {:?}", &path);
    } else {
        let paths = fs::read_dir("../data").unwrap();

        for path in paths {
            let path = path.unwrap().path().display().to_string();
            if !path.ends_with(".txt") {
                continue;
            };
            let graph = parse_input(&path);

            println!("{}", path);
            let solution = lowest_weight(&graph);

            println!("{:?} {}", &solution, path);
        }
    }
}

// find the shortest path in graph from start to end with weight
fn lowest_weight(graph: &Graph) -> bool {
    let mut queue: PriorityQueue<usize, usize> = PriorityQueue::new();
    let mut visited: HashMap<usize, usize> = HashMap::new();

    let invert = 100000;

    queue.push(graph.start, invert);
    visited.insert(graph.start, 0);

    while !queue.is_empty() {
        let (node, weight) = queue.pop().unwrap();
        if node == graph.end {
            if (invert - weight) > 0 {
                return true;
            }
        }

        for edge in &graph.edges {
            if edge.from == node {
                if !visited.contains_key(&edge.to) {
                    queue.push(edge.to, weight - graph.nodes[edge.to].weight);
                    visited.insert(edge.to, weight - graph.nodes[edge.to].weight);
                }
            }
        }
    }

    false
}

fn parse_input(file: &str) -> Graph {
    // read file
    let mut file = File::open(file).expect("file not found");

    // read
    let mut lines = String::new();
    file.read_to_string(&mut lines)
        .expect("something went wrong reading the file");

    let mut lines = lines.lines();
    let first_line = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let second_line = lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let number_of_nodes = first_line[0];
    let number_of_edges = first_line[1];
    let start_node = second_line[0];
    let end_node = second_line[1];

    let mut edges = Vec::with_capacity(number_of_edges);
    let mut nodes = Vec::with_capacity(number_of_nodes);

    let mut nodes_map = HashMap::new();

    for index in 0..number_of_nodes {
        let node: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        nodes_map.insert(node[0].clone(), index);
        nodes.push(Node {
            index: index,
            weight: if node.get(1).is_some() { 1 } else { 0 },
        });
    }

    let start_node = *nodes_map.get(start_node).unwrap();
    let end_node = *nodes_map.get(end_node).unwrap();

    for _ in 0..number_of_edges {
        let edge: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let node1 = nodes_map.get(&edge[0]).unwrap();
        let node2 = nodes_map.get(&edge[2]).unwrap();
        edges.push(Edge {
            from: *node1,
            to: *node2,
        });
        edges.push(Edge {
            from: *node2,
            to: *node1,
        });
    }

    Graph {
        start: start_node,
        end: end_node,
        nodes: nodes,
        edges: edges,
    }
}
