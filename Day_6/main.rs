use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::io::BufRead;

fn check(vec: Vec<char>) -> bool {
    true
}

fn find_marker(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 4)
        .map(|pos| pos + 4)
}

fn step1() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();
    let line = lines.next().unwrap().unwrap();

    println!("{:?}", find_marker(&line[..]));
}

fn find_marker2(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(14)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == 14)
        .map(|pos| pos + 14)
}

fn step2(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);

    let mut lines = buf_reader.lines();
    let line = lines.next().unwrap().unwrap();

    println!("{:?}", find_marker2(&line[..]));
}

fn main(){
    step1();
    step2();
}