use std::str::FromStr;
use std::cmp::{PartialOrd, PartialEq};
use std::default::Default;


// assuming the selections are not more than 1byte
#[derive(Debug, PartialEq, Default, PartialOrd)]
pub struct Selection {
    pub red: usize,
    pub blue: usize,
    pub green: usize,
}

impl FromStr for Selection {
    type Err = &'static str;
    
    // this function takes in a single game instance
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue: usize = 0;
        let mut red: usize = 0;
        let mut green: usize = 0;

        let parts: Vec<&str> = s.trim().split(',').collect();
        for part in parts {
            // let whole_selection: Vec<&str> = part.split(" ").collect();
            let whole_selection: Vec<_> = part.trim().split_whitespace().collect();
            let selection_number = whole_selection.get(0).unwrap().parse::<usize>().unwrap();
            match *whole_selection.get(1).unwrap() {
                "blue" => blue = selection_number,
                "red" => red = selection_number,
                "green" => green = selection_number,
                _ => println!("Error parsing input")
            }
        }

        Ok( Self {
            red: red,
            blue: blue,
            green: green
        })
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
        selections.push(selection.trim().to_string());
    }
    selections
}

pub fn part1(target_selection: Selection) {
    let input_data = include_str!("/home/dru/Rust_projects/aoc2023/src/data/data2.txt");
    let mut counter = 0; // variable to add up the gameID

    for lines in input_data.lines() {
        let (game_id, color_selections) = split_on_colon(lines);
        let id = game_number(game_id); // returns a u16

        for cs in color_selections.lines() {
            let line_selections = split_on_semicolon(cs.to_string()); // each game_id run
            

            let results: Vec<Selection> = line_selections
                .iter()
                .map(|ss| Selection::from_str(ss.as_str()).expect("Error parsing input"))
                .filter(|ss| (ss.green > target_selection.green) | (ss.red > target_selection.red) | (ss.blue > target_selection.blue))
                .collect();
            
            if results.is_empty() {
                counter += id;
            }
        }
    }
    println!("{:?}", counter);
}

