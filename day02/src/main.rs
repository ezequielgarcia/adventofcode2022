use std::env;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum RPSResult {
    Win,
    Draw,
    Lose,
}

struct Round {
    player: RPS,
    result: RPSResult,
}

fn player_win(player: RPS, opponent: RPS) -> bool {
    (player == RPS::Rock && opponent == RPS::Scissors) ||
    (player == RPS::Paper && opponent == RPS::Rock) ||
    (player == RPS::Scissors && opponent == RPS::Paper)
}

impl Round {
    fn score(&self) -> u32 {
        let s = match self.player {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
        let g = match self.result {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Lose => 0,
        };
        return s + g;
    }
}

fn rps_from_string(s: &str) -> RPS {
    match s {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!("Illegal input"),
    }
}

fn result_from_string(s: &str) -> RPSResult {
    match s {
        "X" => RPSResult::Lose,
        "Y" => RPSResult::Draw,
        "Z" => RPSResult::Win,
        _ => panic!("Illegal input"),
    }
}

fn new_round2(s: &str) -> Round {
    let g: Vec<&str> = s.split(" ").collect();
    let o = rps_from_string(g[0]);
    let r = result_from_string(g[1]);
    let p = if o == RPS::Rock {
        match r {
            RPSResult::Win => RPS::Paper,
            RPSResult::Draw => RPS::Rock,
            RPSResult::Lose => RPS::Scissors,
        }
    } else if o == RPS::Paper {
        match r {
            RPSResult::Win => RPS::Scissors,
            RPSResult::Draw => RPS::Paper,
            RPSResult::Lose => RPS::Rock,
        }
    } else { // opponent Scissors
        match r {
            RPSResult::Win => RPS::Rock,
            RPSResult::Draw => RPS::Scissors,
            RPSResult::Lose => RPS::Paper,
        }
    };
    
    Round {
        player: p,
        result: r,
    }
}

fn new_round(s: &str) -> Round {
    let g: Vec<&str> = s.split(" ").collect();
    let o = rps_from_string(g[0]);
    let p = rps_from_string(g[1]);
    let r = if o == p {
        RPSResult::Draw
    } else if player_win(p, o) {
        RPSResult::Win
    } else {
        RPSResult::Lose
    };

    Round {
        player: p,
        result: r,
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

    // Part 1
    let mut total_score1 = 0;
    let mut total_score2 = 0;

    // Each line in the input is a round
    let split = contents.split("\n");
    for r in split {
        if r.trim().is_empty() {
            continue;
        }
        total_score1 += new_round(r).score();
        total_score2 += new_round2(r).score();
    }

    println!("Total score part1: {}", total_score1);
    println!("Total score part2: {}", total_score2);
}
