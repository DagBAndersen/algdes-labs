use priority_queue::PriorityQueue;
use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    fs::{self, File},
    io::Read,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    index: usize,
    weight: usize,
}

#[derive(Debug, Clone, Copy)]
struct BiEdge {
    one: usize,
    two: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<Node>,
    edges: Vec<BiEdge>,
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
        graph.edges = reduce_edges(graph.edges, graph.start, graph.end);

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

fn reduce_edges(edges: Vec<BiEdge>, start: usize, end: usize) -> Vec<BiEdge> {
    let mut deque: VecDeque<BiEdge> = VecDeque::from(edges);

    let mut count = 0;
    loop {
        if count > 50 {
            println!("break");
            break;
        }
        count += 1;

        let edge1 = deque.pop_front();
        if edge1.is_none() {
            break;
        }
        let edge2 = deque.pop_front();
        if edge2.is_none() {
            deque.push_back(edge1.unwrap());
            break;
        }

        let edge1 = edge1.unwrap();
        let edge2 = edge2.unwrap();

        println!("pop edge1: {:?}", &edge1);
        println!("pop edge2: {:?}", &edge2);

        if edge1.one == edge2.one && edge1.two == edge2.two // reduce duplicate edges
            || edge1.one == edge2.two && edge1.two == edge2.one
            || edge1.two == edge2.one && edge1.two == edge2.two
            || edge1.two == edge2.two && edge1.one == edge2.one
        {
            println!("throw away: {:?}", edge2);
            continue;
        }


        if reduceable(&edge1, &edge2) {
            println!("reduce: {:?}", edge1);
            if let Some(edge) = reduce_skip_start_end(&edge1, &edge2, start, end) {
                println!("reduce edge1: {:?} and edge2: {:?} to {:?}", edge1, edge2, edge);
                deque.push_back(edge);
                continue;
            }
        }

        deque.push_back(edge1);
        deque.push_back(edge2);
    }

    Vec::from(deque)
}

fn reduceable(edge1: &BiEdge, edge2: &BiEdge) -> bool {
    if edge1.one == edge2.one && edge1.two != edge2.two {
        return true;
    }

    if edge1.one == edge2.two && edge1.two != edge2.one {
        return true;
    }

    if edge1.two == edge2.one && edge1.one != edge2.two {
        return true;
    }

    if edge1.two == edge2.two && edge1.one != edge2.one {
        return true;
    }

    false
}

fn reduce(edge1: &BiEdge, edge2: &BiEdge) -> BiEdge {
    if edge1.one == edge2.one && edge1.two != edge2.two {
        return BiEdge {
            one: edge1.two,
            two: edge2.two,
        };
    }
    if edge1.one == edge2.two && edge1.two != edge2.one {
        return BiEdge {
            one: edge1.two,
            two: edge2.one,
        };
    }
    if edge1.two == edge2.one && edge1.one != edge2.two {
        return BiEdge {
            one: edge1.one,
            two: edge2.two,
        };
    }
    if edge1.two == edge2.two && edge1.one != edge2.one {
        return BiEdge {
            one: edge1.one,
            two: edge2.one,
        };
    }

    BiEdge {
        one: 30,
        two: 30,
    }
}

fn reduce_skip_start_end(edge1: &BiEdge, edge2: &BiEdge, start: usize, end: usize) -> Option<BiEdge> {
    if edge1.one == start && edge1.two == end {
        return None
    } else if edge1.one == end && edge1.two == start {
        return None
    } 

    if edge2.one == start && edge2.two == end {
        return None
    } else if edge2.one == end && edge2.two == start {
        return None
    }

    if edge1.one == start && edge2.one == start {
        return None
    } else if edge1.one == start && edge2.two == start {
        return None
    } else if edge1.two == start && edge2.one == start {
        return None
    } else if edge1.two == start && edge2.two == start {
        return None
    }

    if edge1.one == end && edge2.one == end {
        return None
    } else if edge1.one == end && edge2.two == end {
        return None
    } else if edge1.two == end && edge2.one == end {
        return None
    } else if edge1.two == end && edge2.two == end {
        return None
    }

    if edge2.one == start && edge1.one == start {
        return None
    } else if edge2.one == start && edge1.two == start {
        return None
    } else if edge2.two == start && edge1.one == start {
        return None
    } else if edge2.two == start && edge1.two == start {
        return None
    }

    if edge2.one == end && edge1.one == end {
        return None
    } else if edge2.one == end && edge1.two == end {
        return None
    } else if edge2.two == end && edge1.one == end {
        return None
    } else if edge2.two == end && edge1.two == end {
        return None
    }

    Some(reduce(edge1, edge2))
}


// find the shortest path in graph from start to end with weight
fn lowest_weight(graph: &Graph) -> isize {
    let mut queue: PriorityQueue<usize, usize> = PriorityQueue::new();
    let mut visited: HashMap<usize, usize> = HashMap::new();

    queue.push(graph.start, 0);
    visited.insert(graph.start, 0);

    let mut max_weight = usize::MAX;

    while !queue.is_empty() {
        let (node, weight) = queue.pop().unwrap();
        println!("node: {}, weight: {}", node, weight);
        if node == graph.end {
            max_weight = max(max_weight, weight);
        }

        for edge in &graph.edges {
            if edge.one == node {
                if !visited.contains_key(&edge.two) {
                    queue.push(edge.two, weight + graph.nodes[edge.two].weight);
                    visited.insert(edge.two, weight + graph.nodes[edge.two].weight);
                }
            }
        }
    }

    if max_weight == usize::MAX {
        -1
    } else {
        max_weight as isize
    }
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
        edges.push(BiEdge {
            one: *node1,
            two: *node2,
        });
        // edges.push(BiEdge {
        //     one: *node2,
        //     two: *node1,
        // });
    }

    Graph {
        start: start_node,
        end: end_node,
        nodes: nodes,
        edges: edges,
    }
}
