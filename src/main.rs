use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use team493::{create_graph_one, create_graph_three, create_graph_two, parse_file, process_pair, process_pair_three, process_pair_two};

fn main() {
    let mut string = String::new();
    println!(" The file should be of csv format as described in the systems project \
    The program has so many bugs so be careful. if the program has other input files it will crash .");
    println!("The file should be in the current directory of the project from where you want to run.");
    println!("Enter the file name:");
    let mut input = std::io::stdin().read_line(&mut string);
    dbg!(string.clone().trim());
    parse_file(string.trim()).expect("Couldnot work");
}
