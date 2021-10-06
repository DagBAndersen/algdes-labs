use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::{env, vec};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
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
            let (number, pref_list) = line.split_once(':').unwrap();
            let num = number.parse::<i32>().unwrap();
            let prefs: Vec<usize> = pref_list
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            if num % 2 == 0 {
                women_pref.push(prefs)
            } else {
                men_pref.push(prefs)
            }
            continue;
        }
        let (number, name) = line.split_once(' ').unwrap();
        let num = number.parse::<i32>().unwrap();
        if num % 2 == 0 {
            woman_names.push(name);
        } else {
            man_names.push(name);
        }
    }

    //println!("{:?}", man_names);
    //println!("{:?}", woman_names);
    //println!("{:?}", men_pref);
    //println!("{:?}", women_pref);

    let pairs = get_pairs(n, men_pref, women_pref);

    print_pairs(pairs, man_names, woman_names);

    Ok(())
}

fn get_pairs(n: usize, men_pref: Vec<Vec<usize>>, women_pref: Vec<Vec<usize>>) -> Vec<i32> {
    let men_pref = prep_men_pref(men_pref);
    let women_pref = prep_woman_pref(women_pref);

    //println!("man prefs: {:?}", men_pref);
    //println!("womans prefs: {:?}", women_pref);

    let mut woman_partner: Vec<i32> = vec![-1; n];
    let mut men_next: Vec<usize> = vec![0; n];
    let mut free_men: Vec<usize> = (0..n).rev().collect();

    while let Some(free_man) = free_men.pop() {
        //println!("");
        //println!("new man = {}", free_man);

        let man_pref = &men_pref[free_man];

        while let Some(&woman) = man_pref.get(men_next[free_man]) {
            men_next[free_man] = men_next[free_man] + 1;

            let current_man = woman_partner[woman];
            if current_man < 0 {
                //println!("woman {} is now engaged in man {}", woman, free_man);
                woman_partner[woman] = free_man as i32;
                break;
            } else if is_man_better(&women_pref[woman], current_man as usize, free_man) {
                free_men.push(current_man as usize);
                //println!("woman {} prefered {} over {}", woman, free_man, current_man);
                //println!("man {} now free", current_man);
                //println!("woman {} is now engaged in man {}", woman, free_man);
                woman_partner[woman] = free_man as i32;
                break;
            } else {
                //println!("woman {} didn't like man {}", woman, free_man);
            }
        }
    }

    woman_partner
}

fn prep_men_pref(pref: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    pref.iter()
        .map(|a| a.iter().map(|&x| x / 2 - 1).collect())
        .collect()
}

fn prep_woman_pref(pref: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    pref.iter()
        .map(|a| a.iter().map(|&x| (x - 1) / 2).collect())
        .map(invert_prefs)
        .collect()
}

fn invert_prefs(prefs: Vec<usize>) -> Vec<usize> {
    let mut wp = vec![0; prefs.len()];
    for (i, &m) in prefs.iter().enumerate() {
        wp[m] = i;
    }
    wp
}

fn is_man_better(women_pref: &Vec<usize>, current_man: usize, new_man: usize) -> bool {
    women_pref[current_man] > women_pref[new_man]
}

fn print_pairs(woman_partner: Vec<i32>, men_names: Vec<String>, women_names: Vec<String>) {
    for (i, u) in woman_partner.into_iter().enumerate() {
        println!("{} -- {}", men_names[u as usize], women_names[i]);
    }
}
