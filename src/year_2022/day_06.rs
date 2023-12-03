#[macros::aoc_day(default_state)]
struct State;

fn common(lines: &Vec<String>, window_size: usize) -> usize {
    let mut total : usize = 0;
    for line in lines {
        for i in 0..(line.len()-window_size) {
            let subset = &line[i..(i+window_size)];
            let mut unique_count = 0;
            for (idx, c) in subset.chars().enumerate() {
                if subset.rfind(c).unwrap() == idx {
                    unique_count += 1;
                }
            }
            if unique_count == window_size {
                total += i + window_size;
                break;
            }
        }
    }
    total
}

fn puzzle_01(state: State) -> AOCResult {
    common(&state.lines, 4).into()
}

fn puzzle_02(state: State) -> AOCResult {
    common(&state.lines, 14).into()
}




