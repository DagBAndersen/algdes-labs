use std::{collections::VecDeque, fs::File, io::Read};

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
    let file = &args[1];

    println!("{}", file);

    let mut graph = parse_input(&file);

    dbg!(&graph.start);
    dbg!(&graph.end);
    dbg!(&graph.nodes);
    // print line in debug

    dbg!(&graph.edges);
    
    graph_filter_red_black_edges(&mut graph);

    let path = bfs(&graph);

    println!("path: {:?}", &path);

}

fn graph_filter_red_black_edges(graph: &mut Graph) {
    graph.edges.retain(|edge| {
        &graph.nodes[edge.node1].red != &graph.nodes[edge.node2].red
    });
}

// Every other node has to be red. It need to switch between red and black.
fn bfs(graph: &Graph) -> bool {
    let mut visited = vec![false; graph.nodes.len()];
    let mut queue = VecDeque::new();
    let mut path = Vec::new();

    queue.push_back(graph.start);
    visited[graph.start] = true;

    while let Some(node_index) = queue.pop_front()  {
        //let node = &graph.nodes[node_index];
        path.push(node_index);

        if node_index == graph.end {
            return true
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
    let mut edges = Vec::new();
    let mut nodes = Vec::new();

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
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let number_of_nodes = first_line[0];
    let number_of_edges = first_line[1];
    let start_node = second_line[0];
    let end_node = second_line[1];

    for index in 0..number_of_nodes {
        let node: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        nodes.push(Node {
            index: index,
            red: if node.get(1).is_some() { true } else { false },
        });
    }

    for index in 0..number_of_edges {
        let edge: Vec<String> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        edges.push(Edge {
            node1: edge[0].parse::<usize>().unwrap(),
            node2: edge[2].parse::<usize>().unwrap(),
        });
    }

    Graph {
        start: start_node,
        end: end_node,
        nodes: nodes,
        edges: edges,
    }
}
