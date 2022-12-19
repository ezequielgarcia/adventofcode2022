use std::env;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    total: u32,
    calories: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        return self.total;
    }
}

// Factory method to create an Elf
fn new_elf(s: &str) -> Elf {
    let mut c = Vec::new();
    let mut t = 0;
    let split = s.split("\n");
    for s in split {
        let cal: u32 = s.parse().unwrap();
        t += cal;
        c.push(cal);
    }
    // Store total and calories as vector,
    // just to not collapse information.
    return Elf {
        calories: c,
        total: t,
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing input");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Cannot read file!");

    let mut elves = Vec::new();

    // Split Elves calories by \n\n
    let split = contents.split("\n\n");

    // Now let's get all our Elves nicely listed
    for s in split {
        if s.trim().is_empty() {
            continue;
        }
        let e = new_elf(s);
        elves.push(e);
    }

    println!("We have {} elves", elves.len());

    // Find max Elf
    let max = elves.iter().max().unwrap();
    println!("Max is {}", max.total_calories());

    // Now sort those Elves!
    elves.sort();

    // Top three
    let mut top_three = 0;
    for n in 1..4 {
        top_three += elves[elves.len() - n].total_calories();
    }
    println!("Top three is {}", top_three);
}
