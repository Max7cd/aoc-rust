#[macros::aoc_day(default_state)]
struct State;

fn puzzle_01(state: State) -> AOCResult {
    let mut instruction_phase = false;
    let mut stacks : Vec<Vec<char>> = vec![];
    for line in state.lines.iter() {
        if !instruction_phase {
            if line.is_empty() {
                instruction_phase = true;
            } else {
                let chars : Vec<char> = line.chars().collect();
                let chunks = chars.chunks(4);
                for (idx, chunk) in chunks.clone().enumerate() {
                    if idx >= stacks.len() {
                        stacks.push(vec![]);
                    }

                    let c = chunk[1].clone();
                    if c.is_alphabetic() {
                        stacks[idx].insert(0, c);
                    }
                }
            }
        } else {
            let mut split = line.split(" from ");
            let amount : i64 = split.next().unwrap()[5..].parse().unwrap();
            let mut split_2 = split.next().unwrap().split(" to ");
            let from : usize = split_2.next().unwrap().parse().unwrap();
            let to : usize = split_2.next().unwrap().parse().unwrap();

            for _ in 0..amount {
                let s = stacks[from-1].pop().unwrap();
                stacks[to-1].push(s);
            }
        }
    }

    stacks.iter().map(|it| it.last().unwrap()).collect::<String>().into()
}

fn puzzle_02(state: State) -> AOCResult {
    let mut instruction_phase = false;
    let mut stacks : Vec<Vec<char>> = vec![];
    for line in state.lines.iter() {
        if !instruction_phase {
            if line.is_empty() {
                instruction_phase = true;
            } else {
                let chars : Vec<char> = line.chars().collect();
                let chunks = chars.chunks(4);
                for (idx, chunk) in chunks.clone().enumerate() {
                    if idx >= stacks.len() {
                        stacks.push(vec![]);
                    }

                    let c = chunk[1].clone();
                    if c.is_alphabetic() {
                        stacks[idx].insert(0, c);
                    }
                }
            }
        } else {
            let mut split = line.split(" from ");
            let amount : i64 = split.next().unwrap()[5..].parse().unwrap();
            let mut split_2 = split.next().unwrap().split(" to ");
            let from : usize = split_2.next().unwrap().parse().unwrap();
            let to : usize = split_2.next().unwrap().parse().unwrap();

            let remaining = stacks[from-1].len() - amount as usize;
            let s = &stacks.clone()[from-1][(remaining)..];
            stacks[to-1].extend_from_slice(s);
            stacks[from-1].truncate(remaining);
        }
    }

    stacks.iter().map(|it| it.last().unwrap()).collect::<String>().into()
}
