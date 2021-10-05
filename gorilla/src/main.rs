use std::{cmp::max, env, fs::File, io::Read};

/*
    Blosum

   A  R  N  D  C  Q  E  G  H  I  L  K  M  F  P  S  T  W  Y  V  B  Z  X  *
A  4 -1 -2 -2  0 -1 -1  0 -2 -1 -1 -1 -1 -2 -1  1  0 -3 -2  0 -2 -1  0 -4
R -1  5  0 -2 -3  1  0 -2  0 -3 -2  2 -1 -3 -2 -1 -1 -3 -2 -3 -1  0 -1 -4
N -2  0  6  1 -3  0  0  0  1 -3 -3  0 -2 -3 -2  1  0 -4 -2 -3  3  0 -1 -4
D -2 -2  1  6 -3  0  2 -1 -1 -3 -4 -1 -3 -3 -1  0 -1 -4 -3 -3  4  1 -1 -4
C  0 -3 -3 -3  9 -3 -4 -3 -3 -1 -1 -3 -1 -2 -3 -1 -1 -2 -2 -1 -3 -3 -2 -4
Q -1  1  0  0 -3  5  2 -2  0 -3 -2  1  0 -3 -1  0 -1 -2 -1 -2  0  3 -1 -4
E -1  0  0  2 -4  2  5 -2  0 -3 -3  1 -2 -3 -1  0 -1 -3 -2 -2  1  4 -1 -4
G  0 -2  0 -1 -3 -2 -2  6 -2 -4 -4 -2 -3 -3 -2  0 -2 -2 -3 -3 -1 -2 -1 -4
H -2  0  1 -1 -3  0  0 -2  8 -3 -3 -1 -2 -1 -2 -1 -2 -2  2 -3  0  0 -1 -4
I -1 -3 -3 -3 -1 -3 -3 -4 -3  4  2 -3  1  0 -3 -2 -1 -3 -1  3 -3 -3 -1 -4
L -1 -2 -3 -4 -1 -2 -3 -4 -3  2  4 -2  2  0 -3 -2 -1 -2 -1  1 -4 -3 -1 -4
K -1  2  0 -1 -3  1  1 -2 -1 -3 -2  5 -1 -3 -1  0 -1 -3 -2 -2  0  1 -1 -4
M -1 -1 -2 -3 -1  0 -2 -3 -2  1  2 -1  5  0 -2 -1 -1 -1 -1  1 -3 -1 -1 -4
F -2 -3 -3 -3 -2 -3 -3 -3 -1  0  0 -3  0  6 -4 -2 -2  1  3 -1 -3 -3 -1 -4
P -1 -2 -2 -1 -3 -1 -1 -2 -2 -3 -3 -1 -2 -4  7 -1 -1 -4 -3 -2 -2 -1 -2 -4
S  1 -1  1  0 -1  0  0  0 -1 -2 -2  0 -1 -2 -1  4  1 -3 -2 -2  0  0  0 -4
T  0 -1  0 -1 -1 -1 -1 -2 -2 -1 -1 -1 -1 -2 -1  1  5 -2 -2  0 -1 -1  0 -4
W -3 -3 -4 -4 -2 -2 -3 -2 -2 -3 -2 -3 -1  1 -4 -3 -2 11  2 -3 -4 -3 -2 -4
Y -2 -2 -2 -3 -2 -1 -2 -3  2 -1 -1 -2 -1  3 -3 -2 -2  2  7 -1 -3 -2 -1 -4
V  0 -3 -3 -3 -1 -2 -2 -3 -3  3  1 -2  1 -1 -2 -2  0 -3 -1  4 -3 -2 -1 -4
B -2 -1  3  4 -3  0  1 -1  0 -3 -4  0 -3 -3 -2  0 -1 -4 -3 -3  4  1 -1 -4
Z -1  0  0  1 -3  3  4 -2  0 -3 -3  1 -1 -3 -1  0 -1 -3 -2 -2  1  4 -1 -4
X  0 -1 -1 -1 -2 -1 -1 -1 -1 -1 -1 -1 -1 -1 -2  0  0 -2 -1 -1 -1 -1 -1 -4
* -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4 -4  1
*/

fn blosum_name_to_index(name: &char) -> usize {
    match name {
        'A' => 0,
        'R' => 1,
        'N' => 2,
        'D' => 3,
        'C' => 4,
        'Q' => 5,
        'E' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'L' => 10,
        'K' => 11,
        'M' => 12,
        'F' => 13,
        'P' => 14,
        'S' => 15,
        'T' => 16,
        'W' => 17,
        'Y' => 18,
        'V' => 19,
        'B' => 20,
        'Z' => 21,
        'X' => 22,
        '*' => 23,
        _ => panic!("Invalid blosum name"),
    }
}

fn get_blosum_matrix() -> [[i32; 24]; 24] {
    // Read from file
    let mut matrix: [[i32; 24]; 24] = [[0; 24]; 24];
    let mut file = File::open("blosum.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    for (i, line) in lines.iter().skip(1).enumerate() {
        // Copilot forgot `skip(1)`
        let mut chars = line.split_whitespace().skip(1);
        for j in 0..24 {
            matrix[i][j] = chars.next().unwrap().parse::<i32>().unwrap();
        }
    }
    matrix
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let fasta = parse_fasta(&args[1]);

    for (name1, protein1) in fasta.iter() {
        for (name2, protein2) in fasta.iter().filter(|&(n, _)| n != name1) {
            let (score, a, b) = align(&protein1, &protein2, -4);
            println!("{}--{}:{}", name1, name2, score);
            println!("{}", a);
            println!("{}", b);
            println!("")
        }
    }
}

fn cost_function(matrix: &[[i32; 24]; 24], a: &char, b: &char) -> i32 {
    matrix[blosum_name_to_index(a)][blosum_name_to_index(b)]
}

// Needleman–Wunsch algorithm - Sequence alignment
fn align(seq1: &str, seq2: &str, alignment_cost: i32) -> (i32, String, String) {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];
    let mut backtrack: Vec<Vec<i32>> = vec![vec![0; seq2.len() + 1]; seq1.len() + 1];

    for i in 1..seq1.len() + 1 {
        dp[i][0] = i as i32 * alignment_cost;
        backtrack[i][0] = 1;
    }

    for j in 1..seq2.len() + 1 {
        dp[0][j] = j as i32 * alignment_cost;
        backtrack[0][j] = 2;
    }

    let matrix = get_blosum_matrix();

    for i in 1..seq1.len() + 1 {
        for j in 1..seq2.len() + 1 {
            let cost = cost_function(
                &matrix,
                &seq1.chars().nth(i - 1).unwrap(),
                &seq2.chars().nth(j - 1).unwrap(),
            );

            let diag = dp[i - 1][j - 1] + cost;
            let up = dp[i - 1][j] + alignment_cost;
            let left = dp[i][j - 1] + alignment_cost;

            dp[i][j] = max(diag, max(up, left));

            if dp[i][j] == diag {
                backtrack[i][j] = 3;
            } else if dp[i][j] == up {
                backtrack[i][j] = 1;
            } else {
                backtrack[i][j] = 2;
            }
        }
    }

    let score = dp[seq1.len()][seq2.len()];
    // println!("{} {} {}", seq1, seq2, score);

    // for x in dp.iter() {
    //     println!("{:?}", x);
    // }
    // println!("");
    // for x in backtrack.iter() {
    //     println!("{:?}", x);
    // }

    let mut i = seq1.len();
    let mut j = seq2.len();
    let mut seq1_alignment = String::new();
    let mut seq2_alignment = String::new();

    while i > 0 && j > 0 {
        if backtrack[i][j] == 3 {
            seq1_alignment.push(seq1.chars().nth(i - 1).unwrap());
            seq2_alignment.push(seq2.chars().nth(j - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if backtrack[i][j] == 2 {
            // Copilot wrote 1 instead of 2
            seq1_alignment.push('-');
            seq2_alignment.push(seq2.chars().nth(j - 1).unwrap());
            j -= 1;
        } else {
            seq1_alignment.push(seq1.chars().nth(i - 1).unwrap());
            seq2_alignment.push('-');
            i -= 1;
        }
    }

    while i > 0 {
        seq1_alignment.push(seq1.chars().nth(i - 1).unwrap());
        seq2_alignment.push('-');
        i -= 1;
    }

    while j > 0 {
        seq1_alignment.push('-');
        seq2_alignment.push(seq2.chars().nth(j - 1).unwrap());
        j -= 1;
    }

    (
        score,
        seq1_alignment.chars().rev().collect(),
        seq2_alignment.chars().rev().collect(),
    )
}

/*

    input file:
    >Human 2144721 HBHU 4HHB
    MVHLTPEEKSAVTALWGKVNVDEVGGEALGRLLVVYPWTQRFFESFGDLSTPDAVMGNPKVKAHGKKVLG
    AFSDGLAHLDNLKGTFATLSELHCDKLHVDPENFRLLGNVLVCVLAHHFGKEFTPPVQAAYQKVVAGVAN
    ALAHKYH
*/

fn parse_fasta(file: &str) -> Vec<(String, String)> {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut new_vec = Vec::new();
    for x in contents.split('>').filter(|x| !x.is_empty()) {
        let lines = x.split_once("\n").unwrap();

        let name = lines.0.split_once(" ").unwrap().0.to_string();

        let seq = lines.1.replace("\n", "");
        new_vec.push((name, seq));
    }
    new_vec
}
