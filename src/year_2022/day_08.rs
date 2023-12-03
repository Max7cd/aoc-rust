#[macros::aoc_day]
struct State {
    map : Vec<Vec<u32>>
}

fn setup(puzzle_input: Vec<String>) -> State {
    State { map : puzzle_input.into_iter().map(|it| it.chars().map(|it| it.to_digit(10).unwrap()).collect::<Vec<_>>()).collect() }
}


fn puzzle_01(state: State) -> AOCResult {
    let mut hidden_count = 0;
    let width = state.map.len();
    let height = state.map.first().unwrap().len();
    for (y, row) in state.map.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let mut hidden_left = false;
            for i in 0..x {
                if state.map[y][i] >= *tree {
                    hidden_left = true;
                    break;
                }
            }

            let mut hidden_top = false;
            for i in 0..y {
                if state.map[i][x] >= *tree {
                    hidden_top = true;
                    break;
                }
            }

            let mut hidden_right = false;
            for i in (x+1)..width {
                if state.map[y][i] >= *tree {
                    hidden_right = true;
                    break;
                }
            }

            let mut hidden_bottom = false;
            for i in (y+1)..height {
                if state.map[i][x] >= *tree {
                    hidden_bottom = true;
                    break;
                }
            }
            if hidden_left && hidden_top && hidden_right && hidden_bottom {
                hidden_count += 1;
            }
        }
    }
    (width * height - hidden_count).into()
}

fn puzzle_02(state: State) -> AOCResult {
    let mut best_score = 0;
    let width = state.map.len();
    let height = state.map.first().unwrap().len();
    for (y, row) in state.map.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let mut left_score = 0;
            for i in (0..x).rev() {
                left_score += 1;
                if state.map[y][i] >= *tree {
                    break;
                }
            }

            let mut top_score = 0;
            for i in (0..y).rev() {
                top_score += 1;
                if state.map[i][x] >= *tree {
                    break;
                }
            }

            let mut right_score = 0;
            for i in (x+1)..width {
                right_score += 1;
                if state.map[y][i] >= *tree {
                    break;
                }
            }

            let mut bottom_score = 0;
            for i in (y+1)..height {
                bottom_score += 1;
                if state.map[i][x] >= *tree {
                    break;
                }
            }

            let score = left_score * top_score * right_score * bottom_score;
            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score.into()
}

