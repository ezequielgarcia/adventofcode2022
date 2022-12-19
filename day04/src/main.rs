use std::env;
use std::fs;
use std::collections::HashSet;
use std::str::FromStr;

fn range_to_set(s: &str) -> HashSet<u32> {
    let p = s.split_once("-").unwrap();
    let range = std::ops::Range { start: u32::from_str(p.0).unwrap(),
                                  end:   u32::from_str(p.1).unwrap() + 1 };
    HashSet::from_iter(range)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input");
        return;
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Cannot read file!");

    // Inspired by Emilio, use iterator instead of modeling
    // the data types as classes with methods.

    // Part 1

    let contained = contents.lines() // -> 5-96,6-99
                        .map(|x| x.split_once(",").unwrap()) // -> (5-96,6-99)
                        .map(|(x, y)| -> (HashSet<u32>, HashSet<u32>) { // -> (Set,Set)
                            (range_to_set(x), range_to_set(y))
                        }) 
                        .filter(|(x,y)| x.is_subset(&y) || y.is_subset(&x))
                        .count();
    println!("Found {} fully contained pairs", contained);

    // Part 2
    let overlapping = contents.lines() // -> 5-96,6-99
                        .map(|x| x.split_once(",").unwrap()) // -> (5-96,6-99)
                        .map(|(x, y)| -> (HashSet<u32>, HashSet<u32>) { // -> (Set,Set)
                            (range_to_set(x), range_to_set(y))
                        }) 
                        .filter(|(x,y)| ! x.is_disjoint(&y))
                        .count();
    println!("Found {} overlapping pairs", overlapping);
}
