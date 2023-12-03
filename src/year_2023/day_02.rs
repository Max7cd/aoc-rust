#[macros::aoc_day]
struct State {
    games: Vec<Game>
}

#[derive(Clone)]
struct Game {
    id: i32,
    rounds: Vec<Round>
}

#[derive(Clone)]
struct Round {
    red: i32,
    green: i32,
    blue: i32
}

fn setup(puzzle_input: Vec<String>) -> State {
    let games = puzzle_input.into_iter().map(|line| {
        split_str!(line, ": ", game_str: String, rounds_str: String);
        let game_id : i32 = game_str[5..].parse().unwrap();
        let mut rounds: Vec<Round> = vec![];
        for round in rounds_str.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color_count in round.split(", ") {
                split_str!(color_count, " ", value: i32, color: String);
                match &color[..] {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => panic!()
                }
            }
            rounds.push(Round { red, green, blue })
        }
        Game { id: game_id, rounds }
    }).collect();
    State { games }
}

fn puzzle_01(state: State) -> AOCResult {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    state.games.into_iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= max_red 
             && round.green <= max_green
             && round.blue <= max_blue
            })
        })
        .map(|it| it.id)
        .sum::<i32>()
        .into()
}

fn puzzle_02(state: State) -> AOCResult {
    state.games.into_iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            game.rounds.iter().for_each(|round| {
                if round.red > max_red {
                    max_red = round.red;
                }
                if round.green > max_green {
                    max_green = round.green;
                }
                if round.blue > max_blue {
                    max_blue = round.blue;
                }
            });
            max_red * max_green * max_blue
        })
        .sum::<i32>()
        .into()
}


