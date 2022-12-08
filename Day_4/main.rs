use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Range;

fn fully_contains(r1: Range<i32>, r2: Range<i32>) -> bool{
    if r1.start <= r2.start && r1.end >= r2.end{
        true
    }else if r2.start <= r1.start && r2.end >= r1.end {
        true
    }else{
        false
    }
}
fn overlap(r1: Range<i32>, r2: Range<i32>) -> bool{
    if r1.start <= r2.start && r2.start <= r1.end{
        true
    }else if r2.start <= r1.start && r1.start <= r2.end {
        true
    }else{
        false
    }
}

fn step1(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    let mut score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();

        let arr = line.split(",").collect::<Vec<&str>>();
        

        let nbs = arr[0].split("-").collect::<Vec<&str>>();
        let elf1 = nbs[0].parse::<i32>().unwrap()..nbs[1].parse::<i32>().unwrap();

        let nbs = arr[1].split("-").collect::<Vec<&str>>();
        let elf2 = nbs[0].parse::<i32>().unwrap()..nbs[1].parse::<i32>().unwrap();

        if fully_contains(elf1, elf2){
            score += 1;
        } else {
            println!("{line}");
        }
    }
    println!("Final score: {score}");
}
fn step2(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    let mut score = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();

        let arr = line.split(",").collect::<Vec<&str>>();
        

        let nbs = arr[0].split("-").collect::<Vec<&str>>();
        let elf1 = nbs[0].parse::<i32>().unwrap()..nbs[1].parse::<i32>().unwrap();

        let nbs = arr[1].split("-").collect::<Vec<&str>>();
        let elf2 = nbs[0].parse::<i32>().unwrap()..nbs[1].parse::<i32>().unwrap();

        if overlap(elf1, elf2){
            println!("{line}");
            score += 1;
        } 
    }
    println!("Final score: {score}");
}

fn main(){
    step1();
    step2();
}