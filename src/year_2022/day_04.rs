#[macros::aoc_day]
pub struct State {
    ranges: Vec<(Range, Range)>
}

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64
}

fn range_contains(r1: &Range, r2: &Range) -> bool {
    r1.start <= r2.start && r1.end >= r2.end
}

fn range_overlaps(r1: &Range, r2: &Range) -> bool {
    r2.start >= r1.start && r2.start <= r1.end || r2.end >= r1.start && r2.end <= r1.end
}

fn setup(puzzle_input: Vec<String>) -> State {
    let ranges: Vec<_> = puzzle_input.iter()
        .map(|it| { it.split(",").into_iter().map(|i| { i.to_string() }).collect::<Vec<_>>().clone()})
        .map(|it| { (it.clone().first().unwrap().clone(), it.clone().last().unwrap().clone()) })
        .map(|it| { (it.0.split("-").into_iter().map(|i| { i.to_string() }).collect::<Vec<_>>(), it.1.split("-").into_iter().map(|i| { i.to_string() }).collect::<Vec<_>>())})
        .map(|it| { (
                Range { start: it.0.first().unwrap().parse().unwrap(), end: it.0.last().unwrap().parse().unwrap() },
                Range { start: it.1.first().unwrap().parse().unwrap(), end: it.1.last().unwrap().parse().unwrap() }) })
                .collect();
    State { ranges }
}

fn puzzle_01(state: State) -> AOCResult {
    state.ranges.iter()
        .filter(|it| { range_contains(&it.0, &it.1) || range_contains(&it.1, &it.0)})
        .count()
        .into()
}

fn puzzle_02(state: State) -> AOCResult {
    state.ranges.iter()
        .filter(|it| { range_overlaps(&it.0, &it.1) || range_overlaps(&it.1, &it.0)})
        .count()
        .into()
}

