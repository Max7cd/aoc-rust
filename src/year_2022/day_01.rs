#[macros::aoc_day]
struct State {
    elves: Vec<i64>
}

fn setup(puzzle_input: Vec<String>) -> State {
    let elves = puzzle_input.into_iter().fold((vec![], 0), |mut acc, line| {
        match line.parse::<i64>() {
            Ok(val) => { acc.1 += val; acc },
            Err(_) => { acc.0.push(acc.1); acc.1 = 0; acc } // newline
        }
    }).0;
    State { elves }
}

fn puzzle_01(state: State) -> AOCResult {
    let max_elf = state.elves.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    max_elf.1.into()
}

fn puzzle_02(mut state: State) -> AOCResult {
    state.elves.sort();
    state.elves.reverse();
    let max_elves_sum = state.elves[..3].iter().sum::<i64>();
    max_elves_sum.into()
}
