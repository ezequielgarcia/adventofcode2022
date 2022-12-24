use std::env;
use std::fs;
use std::collections::{BTreeMap, VecDeque};
use scan_fmt::scan_fmt;

const INPUT_COLUMN_WIDTH: u32 = 4;

//
// Why am I so goddam OO? :(
//

#[derive(Clone)]
struct Crate {
    letter: char,
}

#[derive(Clone)]
struct Column {
    stack: VecDeque<Crate>,
}

struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Column {
    fn new() -> Column {
        Column { stack: VecDeque::new() }
    }

    fn add(&mut self, c: Crate) {
        self.stack.push_front(c);
    }
}

#[derive(Clone)]
struct Columns {
    // Use a BTreeMap because iteration is ordered,
    // and it simplifies getting the top word.
    columns: BTreeMap<u32, Column>,
}

impl Columns {
    fn new() -> Columns {
        Columns {
            columns: BTreeMap::new(),
        }
    }

    fn _grow(&mut self, pos: u32) {
        if ! self.columns.contains_key(&pos) {
            self.columns.insert(pos, Column::new() );
        }
    }

    fn add(&mut self, pos: u32, c: Crate) {
        self._grow(pos);
        self.columns.get_mut(&pos).unwrap().add(c);
    }

    fn _move(&mut self, from: u32, to: u32) {
        // Ah, the borrower checker...!
        let c = {
            self.columns.get_mut(&from).unwrap().stack.pop_back().unwrap()
        };
        self.columns.get_mut(&to).unwrap().stack.push_back(c);
    }

    fn _move2(&mut self, count: u32, from: u32, to: u32) {
        let mut v = {
            let from = &mut self.columns.get_mut(&from).unwrap().stack;
            // [ M, C, D] if count 2 -> [ M ] and [ C, D ]
            from.split_off(from.len() - count as usize)
        };
        self.columns.get_mut(&to).unwrap().stack.append(&mut v);
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            self._move(m.from, m.to);
        }
    }

    fn apply_move2(&mut self, m: &Move) {
        self._move2(m.count, m.from, m.to);
    }

    fn get_top(self) -> String {
        self.columns.iter()
                    .fold("".to_string(),
                          |cur, (_,v)| cur + &v.stack.back().unwrap().letter.to_string())
    }
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

    // Our input has both crates and moves
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let in_crates = parts[0];
    let in_moves = parts[1];

/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
*/
    // We could also use a regex or a parser like nom,
    // but let's do it manually for now.
    let mut columns = Columns::new();
    for row in in_crates.lines() {
        let mut i = 0;
        for letter in row.chars() {
            if letter.is_alphabetic() {
                // New crate, add it to the right column
                let pos = i / INPUT_COLUMN_WIDTH + 1;

                columns.add(pos, Crate { letter });
            }
            i = i + 1;
        }
    }

    let mut columns2 = columns.clone();

    let mut moves: Vec<Move> = Vec::new();
    for row in in_moves.lines() {
        let (count, from, to) = scan_fmt!(row, "move {} from {} to {}\n", u32, u32, u32).unwrap();
        moves.push(Move { count, from, to });
    }

    //      [front, back ]
    // n -> [.. , .. , ..]
    // 1 -> [Z, N]
    // 2 -> [M, C, D]
    //
    // Now play the game!
    moves.iter().for_each(|m| columns.apply_move(m));
    println!("The top is {}", columns.get_top());

    moves.iter().for_each(|m| columns2.apply_move2(m));
    println!("CrateMover 9001! top is now {}", columns2.get_top());
}
