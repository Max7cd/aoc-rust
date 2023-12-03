use std::collections::HashMap;
use clap::Parser;

mod framework;
use framework::AOCDay;

macros::aoc_mod_setup!();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    year: u16,
    day: u8,
}

fn main() {
    let args = Args::parse();
    
    let puzzles: HashMap<String, fn() -> Box<dyn AOCDay>> = load_puzzles();

    let selected_puzzle = format!("{}-{:02}", args.year, args.day);
    if let Some(puzzle_new_fn) = puzzles.get(&selected_puzzle) {
        let puzzle = puzzle_new_fn();
        println!("Puzzle output for {selected_puzzle}:");
        
        println!("Puzzle 01: {}", match puzzle.puzzle_01() {
            Some(s) => s,
            None => String::from("Not implemented yet!")
        });
        println!("Puzzle 02: {}", match puzzle.puzzle_02() {
            Some(s) => s,
            None => String::from("Not implemented yet!")
        });
    } else {
        println!("Puzzle is not implemented yet!")
    }
}
