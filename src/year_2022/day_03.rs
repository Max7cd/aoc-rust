#[macros::aoc_day(default_state)]
struct State;

fn puzzle_01(state: State) -> AOCResult {
    let mut total : u64 = 0;
    for l in state.lines.iter() {
        let mut priorities : u64 = 0;
        let (v1, v2) = l.split_at(l.len()/2);
        let b1 = v1.as_bytes();
        let b2 = v2.as_bytes();
        for x in b1 {
            if b2.contains(x) {
                if x.is_ascii_lowercase() {
                    priorities += 1 + (x - ('a' as u8)) as u64;
                } else {
                    priorities += 27 + (x - ('A' as u8)) as u64;
                }
                break;
            }
        }
        total += priorities;
    }

    total.into()
}

fn puzzle_02(state: State) -> AOCResult {
    let mut total : u64 = 0;
    for l in state.lines.chunks(3) {
        let mut priorities : u64 = 0;
        let v1 = l[0].as_bytes();
        let v2 = l[1].as_bytes();
        let v3 = l[2].as_bytes();
        for x in v1 {
            if v2.contains(x) && v3.contains(x) {
                if x.is_ascii_lowercase() {
                    priorities += 1 + (x - ('a' as u8)) as u64;
                } else {
                    priorities += 27 + (x - ('A' as u8)) as u64;
                }
                break;
            }
        }
        total += priorities;
    }

    total.into()
}

