// a rust program can only have one library, continue from there
use std::str::FromStr;
use std::cmp::{PartialOrd, Ordering, PartialEq};
use std::default::Default;


// #[derive(Debug)]
// struct Games {
//     game_id: usize,
//     selction: Selection
// }

// assuming the selections are not more than 1byte
#[derive(Debug, PartialEq, Default)]
pub struct Selection {
    pub blue: usize,
    pub red: usize,
    pub green: usize,
}

impl FromStr for Selection {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue: usize = 0;
        let mut red: usize = 0;
        let mut green: usize = 0;

        let parts: Vec<&str> = s.trim().split(',').collect();
        for part in parts {
            let whole_selection: Vec<&str> = part.split(" ").collect();
            let selection_number = whole_selection.get(1).unwrap().parse::<usize>().unwrap();
            match *whole_selection.get(0).unwrap() {
                "blue" => blue = selection_number,
                "red" => red = selection_number,
                "green" => green = selection_number,
                _ => return Err("Wrong selection input")
            }
        }

        Ok( Self {
            blue: blue,
            red: red,
            green: green
        })
    }
}

impl PartialOrd for Selection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.red.partial_cmp(&other.red)
    }
}



// use this to return the GameID and selections seperately
fn split_on_colon(input_str: &str) -> (String, String) {
    let whole_string: Vec<&str> = input_str.split(":").collect();
    let v1 = whole_string.get(0).unwrap();
    let v2 = whole_string.get(1).unwrap().trim();
    (v1.to_string(), v2.to_string())
}

// extract number from GameID
fn game_number(game_id: String) -> u16 {
    let whole_string: Vec<&str> = game_id.split(" ").collect();
    let id = whole_string.get(1).unwrap().parse::<u16>().unwrap();
    id
}

// extract specific game selections
fn split_on_semicolon(color_selections: String) -> Vec<String> {
    let whole_string: Vec<&str> = color_selections.split(";").collect();
    let mut selections: Vec<String> = vec![];
    for selection in whole_string {
        selections.push(selection.to_string());
    }
    selections
}

pub fn part1() {
    let input_data = include_str!("/home/dru/Rust_projects/aoc2023/src/data/test_data2.txt");
    for lines in input_data.lines() {
        let (game_id, color_selections) = split_on_colon(lines);
        let id = game_number(game_id); // returns a u16
        let cs = split_on_semicolon(color_selections);
        println!("{:?}", cs);
        
    }
}

