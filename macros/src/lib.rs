#![feature(proc_macro_span)]

extern crate proc_macro;
use std::{path, fs::File, io::Write};

use proc_macro::TokenStream;
use chrono::Datelike;
use proc_macro::Span;

fn extract_year_and_day(source_path: path::PathBuf) -> Option<(i32, i32)> {
    let file_name_string = source_path.to_str().expect("Source path to be a valid String");
    File::create("C:\\Users\\cicib\\Desktop\\aoc_debug_log.txt").unwrap().write_all(file_name_string.as_bytes()).unwrap();
    let file_path = std::path::Path::new(&file_name_string);
    let file = file_path.file_name()?.to_str()?;
    assert!(file.starts_with("day_"));
    let folder = file_path.parent()?.file_name()?.to_str()?;
    assert!(folder.starts_with("year_"));
    let year : i32 = folder[5..9].parse().unwrap();
    let day : i32 = file[4..6].parse().unwrap();

    Some((year, day))
}

#[derive(Default)]
struct Modifiers {
    default_state: bool
}

fn modifiers_from_metadata(metadata: TokenStream) -> Modifiers {
    let metadata_string = metadata.to_string().replace(" ", "");
    let mut modifiers = Modifiers::default();
    for modifier in metadata_string.split(",") {
        match modifier {
            "default_state" => { modifiers.default_state = true },
            _ => {}
        }
    }
    modifiers
}

#[proc_macro_attribute]
pub fn aoc_day(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let source_path: path::PathBuf = Span::call_site().source_file().path();
    let (year, day) = extract_year_and_day(source_path).unwrap_or((0, 0));
    let struct_name = format!("Year{:04}Day{:02}", year, day);
    let modifiers = modifiers_from_metadata(metadata);

    let state_struct_definition = if modifiers.default_state {
        r#"struct State {
            lines: Vec<String>
        }
        
        fn setup(puzzle_input: Vec<String>) -> State {
            State { lines: puzzle_input }
        }"#.to_string()
    } else {
        input.to_string()
    };

    format!(r#"
    use crate::framework::*;
    use crate::split_str;
    
    #[derive(Clone)]
    {state_struct_definition}

    pub struct {struct_name} {{
        state: State
    }}
    
    impl AOCDay for {struct_name} {{
        fn puzzle_01(&self) -> Option<String> {{
            return puzzle_01(self.state.clone()).0;
        }}
        
        fn puzzle_02(&self) -> Option<String> {{
            return puzzle_02(self.state.clone()).0;
        }}

        fn setup() -> Self where Self : Sized {{
            let puzzle_input = open_input_file({year}, {day});
            return Self {{ state: setup(puzzle_input) }};
        }}
    }}
    "#).parse().unwrap()
}

struct YearDayCombo {
    year: i32,
    day: i32
}

#[proc_macro]
pub fn aoc_mod_setup(_: TokenStream) -> TokenStream {
    const FIRST_YEAR : i32 = 2015;
    let current_year = chrono::Utc::now().year();
    let mut puzzles_found: Vec<YearDayCombo> = vec![];
    let mut output = String::new();
    for year in FIRST_YEAR..=current_year {
        if path::Path::new(&format!("./src/year_{year}")).exists() {
            output.push_str(&format!("mod year_{year} {{"));
            for day in 1..=25 {
                let day_str = format!("day_{:02}", day);
                if path::Path::new(&format!("./src/year_{year}/{day_str}.rs")).exists() {
                    puzzles_found.push(YearDayCombo { year, day });
                    output.push_str(format!("  pub use {}::*;\n  pub mod {};\n", day_str, day_str).as_str());
                } 
            }
            output.push_str(format!("}}\nuse year_{year}::*;").as_str());
        }
    }

    output.push_str(r#"
fn load_puzzles() -> HashMap<String, fn() -> Box<dyn AOCDay>> {
    let mut puzzles: HashMap<String, fn() -> Box<dyn AOCDay>> = HashMap::new();"#);

    for year_day_combo in puzzles_found {
        output.push_str(&format!("insert_puzzle!(puzzles, {:04}, {:02});", year_day_combo.year, year_day_combo.day))
    }

    output.push_str(r#"
    return puzzles;
}
    "#);
    
    return output.parse().unwrap();
}