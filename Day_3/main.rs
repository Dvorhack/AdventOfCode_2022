use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn filter_uniq(vec: &str) -> String {
    vec.chars().into_iter()
        .map(|course| course)
        .collect::<HashSet<_>>().into_iter().collect::<String>()
}

fn value(c: char) -> u32 {
    let nb = c as u32;
    if nb > 0x61 {
        nb - 0x61 + 1
    }else{
        nb - 0x41 + 1 + 26
    }
}

fn find_uniq(s1: &String, s2: &String) -> Result<char, ()>{
    for i in s1.chars(){
        if s2.contains(i) {
            return Ok(i);
        }
    }
    Err(())
}

fn find_uniq2(s1: &String, s2: &String, s3: &String) -> Result<char, ()>{
    for i in s1.chars(){
        if s2.contains(i) && s3.contains(i){
            return Ok(i);
        }
    }
    Err(())
}

fn step1(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    let mut somme = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let half_len = line.len() / 2;
        let first = filter_uniq(&line[0..half_len]);
        let second = filter_uniq(&line[half_len..]);

        let uniq = find_uniq(&first,&second).unwrap();
        let val = value(uniq);

        println!("{line} {first:?} {second:?} {uniq} {val}");

        somme += val;
    }
    println!("Somme {somme}");
}

fn step2(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    let mut somme = 0;
    let mut i = 1;
    let mut first = "".to_string();
    let mut second = "".to_string();

    for line in buf_reader.lines() {
        let line = line.unwrap();
        match i {
            1 => { first = filter_uniq(&line); },
            2 => { second = filter_uniq(&line); },
            3 => {
                let third = filter_uniq(&line);
                let uniq = find_uniq2(&first,&second, &third).unwrap();
                let val = value(uniq);
        
                println!("{line} {first:?} {second:?} {uniq} {val}");
        
                somme += val;
                i = 0;
            }
            _ => {}
        }


        i += 1;
    }
    println!("Somme {somme}");
}

fn main(){
    step2();
}