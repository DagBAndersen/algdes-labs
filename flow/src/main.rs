use std::{cmp::min, fs::File, io::Read};

// Ford–Fulkerson algorithm

fn main() {
    // get value from arguments
    let args = std::env::args();
    let filename = args.skip(1).next().unwrap();
    let (mut graph, names) = parse_input(&filename);

    let source = 0;
    let sink = graph.len() - 1;

    let max_flow = ford_fulkerson(&mut graph, source, sink, names);

    println!("Max flow: {}", max_flow);
}

fn parse_input(file_path: &str) -> (Vec<Vec<i32>>, Vec<String>) {
    let mut input = String::new();
    File::open(file_path)
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");

    let mut iterator = input.lines();
    let number_of_nodes: usize = iterator.next().unwrap().parse().unwrap();
    let mut names = Vec::with_capacity(number_of_nodes);
    for _ in 0..number_of_nodes {
        names.push(iterator.next().unwrap().to_string());
    }

    let mut graph: Vec<Vec<i32>> = vec![vec![0; number_of_nodes]; number_of_nodes];
    for line in iterator.skip(1) {
        //println!("{}", line);
        let mut split = line.split_whitespace();
        let node1 = split.next().unwrap().parse::<usize>().unwrap();
        let node2 = split.next().unwrap().parse::<usize>().unwrap();
        let capacity = split.next().unwrap().parse::<i32>().unwrap();
        graph[node1][node2] = if capacity < 0 { i32::MAX / 2 } else { capacity };
        graph[node2][node1] = if capacity < 0 { i32::MAX / 2 } else { capacity };
    }
    (graph, names)  
}

// The timecomplexity of the ford_fulkerson algorithm is 
// Ford–Fulkerson augmenting path algorithm
fn ford_fulkerson(graph: &mut Vec<Vec<i32>>, s: usize, t: usize, names: Vec<String>) -> i32 {
    let mut flow = 0;

    let original_copy = graph.clone();

    while let Some(parent) = bfs(graph, s, t) {
        let mut min_capacity = i32::MAX;

        let mut v = t;
        while v != s {
            let u = parent[v] as usize;
            min_capacity = min(min_capacity, graph[u][v]);
            v = parent[v] as usize;
        }

        let mut v = t;
        while v != s {
            let u = parent[v] as usize;
            graph[u][v] -= min_capacity;
            graph[v][u] += min_capacity;
            v = parent[v] as usize;
        }

        flow += min_capacity;
    }

    print_min_cut_with_names(&original_copy, graph, s, names);
    //print_min_cut(&original_copy, graph, s);

    flow
}

fn print_min_cut(original_graph: &Vec<Vec<i32>>, residual_graph: &Vec<Vec<i32>>, s: usize) {
    let mut visited = vec![false; residual_graph.len()];
    dfs(residual_graph, s, &mut visited);

    for i in 0..residual_graph.len() {
        for j in 0..residual_graph.len() {
            if original_graph[i][j] > 0 && visited[i] && !visited[j] {
                println!("{} {} {}", i, j, original_graph[i][j]);
            }
        }
    }
}

fn print_min_cut_with_names(
    original_graph: &Vec<Vec<i32>>,
    residual_graph: &Vec<Vec<i32>>,
    s: usize,
    names: Vec<String>,
) {
    let mut visited = vec![false; residual_graph.len()];
    dfs(residual_graph, s, &mut visited);

    for i in 0..residual_graph.len() {
        for j in 0..residual_graph.len() {
            if original_graph[i][j] > 0 && visited[i] && !visited[j] {
                println!(
                    "{} -> {} with capacity: {}",
                    names[i], names[j], original_graph[i][j]
                );
            }
        }
    }
}

fn dfs(graph: &Vec<Vec<i32>>, s: usize, visited: &mut Vec<bool>) {
    visited[s] = true;
    for i in 0..graph.len() {
        if graph[s][i] > 0 && !visited[i] {
            dfs(graph, i, visited);
        }
    }
}


// The timecomplexity of bfs is O(V^2) where V is the number of vertices in the graph.
// BFS to find path in graph from s to t (if exists).
fn bfs(graph: &Vec<Vec<i32>>, s: usize, t: usize) -> Option<Vec<i32>> {
    let mut queue: Vec<usize> = vec![s];

    let mut visited = vec![false; graph.len()];
    visited[s] = true;

    let mut parent = vec![0; graph.len()];
    parent[s] = -1;

    while let Some(v) = queue.pop() {
        for i in 0..graph.len() {
            if graph[v][i] > 0 && !visited[i] {
                if i == t {
                    parent[i] = v as i32;
                    return Some(parent);
                }
                queue.push(i);
                visited[i] = true;
                parent[i] = v as i32;
            }
        }
    }
    None
}

fn pretty_print(graph: &Vec<Vec<i32>>) {
    for row in graph.iter() {
        for col in row.iter() {
            print!("{} ", col);
        }
        println!("");
    }
}

#[test]
fn test_ford_fulkerson_simple() {
    let mut graph = vec![
        vec![0, 1, 0, 1, 0, 10],
        vec![0, 0, 0, 2, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 3],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ];

    let source = 0;
    let sink = graph.len() - 1;

    let max_flow = ford_fulkerson(
        &mut graph,
        source,
        sink,
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
        ],
    );
    assert_eq!(max_flow, 12);
}

#[test]
fn test_ford_fulkerson_from_data_file() {
    let (mut graph, names) = parse_input(&"data/rail.txt");

    let source = 0;
    let sink = graph.len() - 1;

    let max_flow = ford_fulkerson(&mut graph, source, sink, names);
    assert_eq!(max_flow, 163);
}
