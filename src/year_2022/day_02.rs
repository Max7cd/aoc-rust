#[macros::aoc_day(default_state)]
struct State;

fn puzzle_01(state: State) -> AOCResult {
    let mut total = 0;
    for l in state.lines.iter() {
        total += match &l[..] {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => { panic!("invalid string") }
        }
    }
    total.into()
}

fn puzzle_02(state: State) -> AOCResult {
    let mut total = 0;
    for l in state.lines.iter() {
        total += match &l[..] {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => { panic!("invalid string") }
        }
    }
    total.into()
}




