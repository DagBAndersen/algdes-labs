use std::{
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::Read,
};

#[derive(Debug)]
struct Node {
    index: usize,
    red: bool,
}

#[derive(Debug)]
struct Edge {
    node1: usize,
    node2: usize,
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

        let mut graph = parse_input(&file);

        dbg!(&graph.start);
        dbg!(&graph.end);
        dbg!(&graph.nodes);

        dbg!(&graph.edges);

        graph_filter_red_black_edges(&mut graph);

        let path = bfs(&graph);

        println!("path: {:?}", &path);
    } else {
        let paths = fs::read_dir("../data").unwrap();

        for path in paths {
            let path = path.unwrap().path().display().to_string();
            if !path.ends_with(".txt") {
                continue;
            };
            let mut graph = parse_input(&path);

            graph_filter_red_black_edges(&mut graph);

            let solution = bfs(&graph);

            println!("{:?} {}", &solution, path);
        }
    }
}

fn graph_filter_red_black_edges(graph: &mut Graph) {
    graph
        .edges
        .retain(|edge| &graph.nodes[edge.node1].red != &graph.nodes[edge.node2].red);
}

// Every other node has to be red. It need to switch between red and black.
fn bfs(graph: &Graph) -> bool {
    let mut visited = vec![false; graph.nodes.len()];
    let mut queue = VecDeque::new();
    let mut path = Vec::new();

    queue.push_back(graph.start);
    visited[graph.start] = true;

    while let Some(node_index) = queue.pop_front() {
        //let node = &graph.nodes[node_index];
        path.push(node_index);

        if node_index == graph.end {
            return true;
        }

        for edge in &graph.edges {
            //let node1 = &graph.nodes[edge.node1];
            //let node2 = &graph.nodes[edge.node2];
            if edge.node1 == node_index && !visited[edge.node2] {
                queue.push_back(edge.node2);
                visited[edge.node2] = true;
            } else if edge.node2 == node_index && !visited[edge.node1] {
                queue.push_back(edge.node1);
                visited[edge.node1] = true;
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
            red: if node.get(1).is_some() { true } else { false },
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
            node1: *node1,
            node2: *node2,
        });
    }

    Graph {
        start: start_node,
        end: end_node,
        nodes: nodes,
        edges: edges,
    }
}
