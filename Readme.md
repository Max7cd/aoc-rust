# running puzzles 

To run you need to add an advent of code access token into your `aoc-config.toml`

It should look something like this:
```
token = "53616c74656..."
```

The token is a very long base16 encoded string and can be extraced from a logged in session in the browser.
It is contained in all request headers to [https://adventofcode.com/](https://adventofcode.com/) in the field `Cookie: session=53616c74656...`

To execute a specific puzzle, you need to use `cargo run -- year day`, for example `cargo run -- 2022 1`

# coding new puzzles

The framework uses the most unhygienic macros you could possibly imagine, but it eliminates a lot of boilerplate code.

to add a new puzzle put a file called `day_XX.rs` where XX is between 01 and 25 into a folder `year_XXXX` where XXXX is a year greater than 2015.
The module setup and adding it to our executable is done automatically.

To solve a puzzle create 3 functions and a struct called `State` with the following signatures:

```rs
#[macros::aoc_day]
struct State {
    
}

fn setup(puzzle_input: Vec<String>) -> State {
    State { }
}

fn puzzle_01(state: State) -> AOCResult {
    not_implemented()
}

fn puzzle_02(mut state: State) -> AOCResult {
    not_implemented()
}
```

# other useful macros and options

## AOCResult

The type that puzzle_01 and puzzle_02 return is AOCResult which implements `From<T>` for any `T` that implements `std::fmt::Display`
This means almost anything can be returned if you use .into()

## default_state

If you do not want to use any custom fields in your state and just want to straight up get the lines as a `Vec<String>`
you can use:

```rs
#[macros::aoc_day(default_state)]
struct State;
```

This generates a struct definition with a field `lines: Vec<String>` and a setup function that fills that field.
It is equivalent to this code:
```rs
#[macros::aoc_day]
struct State{
    lines: Vec<String>
}

fn setup(puzzle_input: Vec<String>) -> State {
    State { lines: puzzle_input }
}
```

## split_str

The `split_str` macro allows you to split a string into multiple parts and automatically assign the correct datatype.
For example: `split_str!(line, ": ", id: i32, values: String)` splits the String `line` into an id of type i32 and another String.

```rs
let line = "11: red, green, yellow";
split_str!(line, ": ", id: i32, values: String)
=> let id: i32 = 11;
=> let values: String = "red, green, yellow";
```