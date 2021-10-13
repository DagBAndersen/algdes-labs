use std::io::Read;

// Ford–Fulkerson algorithm

fn main() {
    // get value from arguments
    let args = std::env::args();
    let filename = args.skip(1).next().unwrap();
    let mut graph = parse_input(&filename);

    let source = 0;
    let sink = graph.len() - 1;

    let max_flow = ford_fulkerson(&mut graph, source, sink);

    println!("Max flow: {}", max_flow);
}

fn parse_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut input = String::new();
    std::fs::File::open(file_path)
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");

    let mut iterator = input.lines();
    let number_of_nodes: usize = iterator.next().unwrap().parse().unwrap();
    let mut graph: Vec<Vec<i32>> = vec![vec![0; number_of_nodes]; number_of_nodes];
    for line in iterator.skip(number_of_nodes + 1) {
        //println!("{}", line);
        let mut split = line.split_whitespace();
        let node1 = split.next().unwrap().parse::<usize>().unwrap();
        let node2 = split.next().unwrap().parse::<usize>().unwrap();
        let capacity = split.next().unwrap().parse::<i32>().unwrap();
        graph[node1][node2] = if capacity < 0 { i32::MAX / 2 } else { capacity };
        graph[node2][node1] = if capacity < 0 { i32::MAX / 2 } else { capacity };
    }
    graph
}

// Ford–Fulkerson augmenting path algorithm
fn ford_fulkerson(graph: &mut Vec<Vec<i32>>, s: usize, t: usize) -> i32 {
    let mut flow = 0;

    let original_copy = graph.clone();

    while let Some(parent) = bst(graph, s, t) {
        let mut min_capacity = i32::MAX;

        let mut v = t;
        while v != s {
            let u = parent[v] as usize;
            min_capacity = std::cmp::min(min_capacity, graph[u][v]);
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

    let mut visited = vec![false; graph.len()];
    dfs(graph, s, &mut visited);

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            if original_copy[i][j] > 0 && visited[i] && !visited[j] {
                println!("{} {} {}", i, j, original_copy[i][j]);
            }
        }
    }
    flow
}

fn dfs(graph: &Vec<Vec<i32>>, s: usize, visited: &mut Vec<bool>) {
    visited[s] = true;
    for i in 0..graph.len() {
        if graph[s][i] > 0 && !visited[i] {
            dfs(graph, i, visited);
        }
    }
}

// BST to find path in graph from s to t (if exists).
fn bst(graph: &Vec<Vec<i32>>, s: usize, t: usize) -> Option<Vec<i32>> {
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

    let max_flow = ford_fulkerson(&mut graph, source, sink);
    assert_eq!(max_flow, 12);
}

#[test]
fn test_ford_fulkerson_from_data_file() {
    let mut graph = parse_input(&"data/rail.txt");

    let source = 0;
    let sink = graph.len() - 1;

    let max_flow = ford_fulkerson(&mut graph, source, sink);
    assert_eq!(max_flow, 163);
}
