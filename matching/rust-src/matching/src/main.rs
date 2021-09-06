use std::{env, vec};

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let file = File::open(args[1].as_str())?;
    let reader = BufReader::new(file);

    let mut n = 0;

    let mut man_names = Vec::new();
    let mut woman_names = Vec::new();

    let mut men_pref = Vec::new();
    let mut women_pref = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if line.starts_with('n') {
            n = line[2..].parse::<usize>().unwrap();
            continue;
        }

        if line.contains(":") {
            let vec = line.split_once(':').unwrap();
            let num = vec.0.parse::<i32>().unwrap();
            //println!("{}", line);
            let name: Vec<usize> = vec.1
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            if num % 2 == 0 {
                women_pref.push(name)
            } else {
                men_pref.push(name)
            }
        } else {
            let vec = line.split(' ').collect::<Vec<&str>>();
            let num = vec[0].parse::<i32>().unwrap();
            let name = vec[1].to_string();
            if num % 2 == 0 {
                woman_names.push(name);
            } else {
                man_names.push(name);
            }
        }
    }

    //println!("{:?}", man_names);
    //println!("{:?}", woman_names);
    //println!("{:?}", men_pref);
    //println!("{:?}", women_pref);

    let pairs = get_pairs(n, &mut men_pref, &mut women_pref);

    print_pairs(pairs, man_names, woman_names);

    Ok(())
}

fn get_pairs(n: usize, men_pref: &mut Vec<Vec<usize>>, women_pref: &mut Vec<Vec<usize>>) -> Vec<i32> {
    let men_pref: Vec<Vec<usize>> = men_pref
        .iter()
        .map(|a| a.iter().map(|x| *x / 2 - 1).collect())
        .collect();
    let women_pref: Vec<Vec<usize>> = women_pref
        .iter()
        .map(|a| a.iter().map(|x| (*x as usize - 1) / 2).collect())
        .map(|a| invert_woman_pref(a))
        .collect();

    //println!("man prefs: {:?}", men_pref);
    //println!("womans prefs: {:?}", women_pref);

    let mut woman_partner: Vec<i32> = vec![-1; n];
    let mut men_next: Vec<usize> = vec![0; n];
    let mut free_men: Vec<usize> = (0..n).rev().collect();

    while let Some(free_man) = free_men.pop() {
        //println!("");
        //println!("new man = {}", free_man);

        let man_pref = &men_pref[free_man as usize];

        while let Some(woman) = man_pref.get(men_next[free_man]) {
            men_next[free_man as usize] = men_next[free_man] + 1;

            let current_man = woman_partner[*woman as usize];
            if current_man < 0 {
                //println!("woman {} is now engaged in man {}", woman, free_man);
                woman_partner[*woman as usize] = free_man as i32;
                break;
            } else if is_man_better(&women_pref[*woman as usize], current_man as usize, free_man) {
                free_men.push(current_man as usize);
                //println!("woman {} prefered {} over {}", woman, free_man, current_man);
                //println!("man {} now free", current_man);
                //println!("woman {} is now engaged in man {}", woman, free_man);
                woman_partner[*woman as usize] = free_man as i32;
                break;
            } else {
               //println!("woman {} didn't like man {}", woman, free_man);
            }
        }
    }

    woman_partner
}

fn is_man_better(women_pref: &Vec<usize>, current_man: usize, new_man: usize) -> bool {
    women_pref[current_man] > women_pref[new_man]
}

fn print_pairs(woman_partner: Vec<i32>, men_names: Vec<String>, women_names: Vec<String>) {
    for (i, u) in woman_partner.into_iter().enumerate() {
        println!(
            "{} -- {}",
            men_names[u as usize], women_names[i]
        );
    }
}

fn invert_woman_pref(women_pref: Vec<usize>) -> Vec<usize> {
    let mut wp = vec![0; women_pref.len()];
    for (i, m) in women_pref.iter().enumerate() {
        wp[*m] = i;
    }
    wp
}
