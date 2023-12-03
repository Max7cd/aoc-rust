use std::collections::HashMap;

#[macros::aoc_day(default_state)]
struct State;

fn puzzle_01(state: State) -> AOCResult {
    state.lines.iter().map(|line| {
        let first = line.chars().find(|it| it.is_digit(10)).unwrap().to_digit(10).unwrap();
        let second = line.chars().rev().find(|it| it.is_digit(10)).unwrap().to_digit(10).unwrap();
        first * 10 + second
    }).sum::<u32>().into()
}

fn puzzle_02(state: State) -> AOCResult {
    let replacements: HashMap<String, char> = HashMap::from([
        ("one".to_owned(), '1'),
        ("two".to_owned(), '2'),
        ("three".to_owned(), '3'),
        ("four".to_owned(), '4'),
        ("five".to_owned(), '5'),
        ("six".to_owned(), '6'),
        ("seven".to_owned(), '7'),
        ("eight".to_owned(), '8'),
        ("nine".to_owned(), '9'), // I should handle this more precisely in replacement, since if there is "ninine" it will not get recognized... lets hope it works without that
    ]);
    state.lines.iter().map(|line| {
        let mut chars: Vec<char> = line.chars().collect();
        let mut candidates : Vec<(Vec<char>, usize)> = replacements.keys().map(|it| (it.chars().collect(), 0)).collect();
        for i in 0..chars.len() {
            for (candidate, burned_length) in candidates.iter_mut() {
                if candidate[*burned_length] == chars[i] {
                    *burned_length += 1; // matches, continue matching
                } else {
                    *burned_length = 0; // does not match, reset
                    if candidate[*burned_length] == chars[i] { // and try again from the first char
                        *burned_length += 1;
                    }
                }
                if *burned_length == candidate.len() {
                    let x = replacements.get(&candidate.iter().collect::<String>()).unwrap();
                    chars[i + 1 - *burned_length] = x.clone();
                    *burned_length = 0;
                }
            }
        }
        chars.into_iter().collect::<String>()
    }).map(|line| {
        let first = line.chars().find(|it| it.is_digit(10)).unwrap().to_digit(10).unwrap();
        let second = line.chars().rev().find(|it| it.is_digit(10)).unwrap().to_digit(10).unwrap();
        first * 10 + second
    }).sum::<u32>().into()
}


