use std::env;
use std::fs;
use std::collections::HashSet;

struct Group {
    members: [Rucksack; 3]
}

#[derive(Clone)]
struct Rucksack {
    items: HashSet<char>,
    compartments: (HashSet<char>, HashSet<char>),
}

impl Group {
    fn find_badge(&self) -> char {
        let b = &(&self.members[0].items & &self.members[1].items) & &self.members[2].items;
        assert_eq!(b.len(), 1);
        return *b.iter().next().unwrap();
    }
}

impl Rucksack {
    fn find_bad_item(&self) -> char {
        let b = &self.compartments.0 & &self.compartments.1;
        assert_eq!(b.len(), 1);
        return *b.iter().next().unwrap();
    }
}

fn item_priority(c: char) -> u32 {
    if c.is_lowercase() {
        u32::from(c) - u32::from('a') + 1
    } else {
        u32::from(c) - u32::from('A') + (u32::from('z') - u32::from('a')) + 2
    }
}

fn new_rucksack(s: &str) -> Rucksack {
    let (one, two) = s.split_at(s.len() / 2);
    Rucksack {
        items: HashSet::from_iter(s.chars()),
        compartments: (HashSet::from_iter(one.chars()),
                       HashSet::from_iter(two.chars())),
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

    // Part 1
    let mut sum_priority = 0;

    // Each line is a rucksack
    let split = contents.split("\n");
    for r in split {
        if r.trim().is_empty() {
            continue;
        }
        let rs = new_rucksack(r);
        sum_priority += item_priority(rs.find_bad_item());

        elves.push(rs);
    }
    println!("{}", sum_priority);

    // Part 2
    let mut groups = Vec::new();

    // Let's loop through the vector, creating the groups
    let mut i = 0;
    loop {
        let g = Group {
            members: [elves[i].clone(),
                      elves[i+1].clone(),
                      elves[i+2].clone(),
            ]
        };
        groups.push(g);
        i += 3;
        if i >= elves.len() {
            break;
        }
    }
    assert_eq!(groups.len(), elves.len() / 3);

    sum_priority = 0;
    for g in groups {
        sum_priority += item_priority(g.find_badge());
    }
    println!("{}", sum_priority);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }
}
