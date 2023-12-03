use std::path::Path;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, BufRead};

use reqwest::StatusCode;

pub trait AOCDay {
    fn puzzle_01(&self) -> Option<String>;
    fn puzzle_02(&self) -> Option<String>;
    fn setup() -> Self where Self : Sized;
}

pub fn open_input_file(year: i64, day: i64) -> Vec<String> {
    let path = format!("inputs/{:04}/{:02}.txt", year, day);
    match read_lines(&path) {
        Ok(lines) => lines.map(|it| it.unwrap()).collect(),
        Err(_) => {
            std::fs::create_dir_all(Path::new(&path).parent().unwrap()).unwrap();
            let content = download_input(year, day); 
            std::fs::write(&path, content).unwrap();
            read_lines(&path).unwrap().map(|it| it.unwrap()).collect()
        },
    }
    
}

#[macro_export]
macro_rules! insert_puzzle {
    ( $hashmap:ident, $year:expr, $day:expr ) => {
        paste::paste! {
            $hashmap.insert(
                format!("{:04}-{:02}", stringify!($year), stringify!($day)), 
                || { Box::new([<Year $year Day $day>]::setup()) }
            );
        }
    };
}


pub fn download_input(year: i64, day: i64) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header("Cookie", format!("session={}", get_aoc_token().unwrap()))
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => response.text().unwrap(),
        s => panic!("Downloading Input returned Status {}: {}", s.as_u16(), response.text().unwrap())
    }
}

#[derive(serde::Deserialize)]
struct Config {
    token: String
}


pub fn get_aoc_token() -> Result<String, ()> {
    let filecontent = read_file("aoc-config.toml").unwrap();
    let config : Config = toml::from_str(&filecontent).unwrap();
    return Ok(config.token);
}

pub struct AOCResult(pub Option<String>);

#[allow(unused)]
pub fn not_implemented() -> AOCResult {
    AOCResult(None)
}

impl<T> From<T> for AOCResult where T : Display {
    fn from(value: T) -> Self {
        AOCResult(Some(format!("{value}")))
    }
}

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(unused)]
pub fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

#[allow(unused)]
#[macro_export]
macro_rules! split_str {
    ( $to_split:expr, $split_on:expr, $( $vname:ident : $vtype:ty ),+ ) => {
        let mut split = $to_split.split($split_on);
        $(let $vname : $vtype = match (split.next().unwrap().parse()) { Ok(x) => x, Err(_) => panic!("failed to unwrap value from input: {}", $to_split)};)+
    };
}