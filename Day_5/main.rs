use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn step2(){
    let file_path = "input.txt";
    const NEW_VEC: Vec<String> = Vec::new();
    let mut stacks: [Vec<String>; 9] = [NEW_VEC; 9];

    for i in 0..9{
        for j in (0..8).rev(){
            let file = File::open(file_path)
                        .expect("file not found!");
            let  buf_reader = BufReader::new(file);
            let cra = buf_reader.lines()
                                .nth(j)
                                .expect("File not 5 line long")
                                .expect("Cannot get line 5")[4*i+1..4*i+2]
                                .to_string();
            if cra != " ".to_string(){
                stacks[i].push(cra);
            }
        }
    }

    println!("Final score: {stacks:?}");

    let file = File::open(file_path)
                    .expect("file not found!");
    let  buf_reader = BufReader::new(file);

    for line in buf_reader.lines().skip(10) {
        let line = line.unwrap();

        let arr = line.split(" ").collect::<Vec<&str>>();

        let nb: u8 = arr[1].parse().unwrap();
        let from: usize = arr[3].parse().unwrap();
        let to: usize = arr[5].parse().unwrap();
        let mut tmp: Vec<String> = Vec::new();

        for i in 0..nb{
            match stacks[from-1].pop(){
                Some(elt) => {tmp.push(elt);},
                None => {}
            }
        }
        for i in 0..nb{
            match tmp.pop(){
                Some(elt) => {stacks[to-1].push(elt);},
                None => {}
            }
        }
        println!("{line:?} {nb} {from} {to}");
        println!("Final score: {stacks:?}");
        // return;

    }
    println!("Final score: {stacks:?}");

    for i in 0..stacks.len(){
        print!("{:?}",stacks[i].pop().unwrap());
    }
    
}
fn step1(){
    let file_path = "input.txt";
    const NEW_VEC: Vec<String> = Vec::new();
    let mut stacks: [Vec<String>; 9] = [NEW_VEC; 9];

    for i in 0..9{
        for j in (0..8).rev(){
            let file = File::open(file_path)
                        .expect("file not found!");
            let  buf_reader = BufReader::new(file);
            let cra = buf_reader.lines()
                                .nth(j)
                                .expect("File not 5 line long")
                                .expect("Cannot get line 5")[4*i+1..4*i+2]
                                .to_string();
            if cra != " ".to_string(){
                stacks[i].push(cra);
            }
        }
    }

    println!("Final score: {stacks:?}");

    let file = File::open(file_path)
                    .expect("file not found!");
    let  buf_reader = BufReader::new(file);

    for line in buf_reader.lines().skip(10) {
        let line = line.unwrap();

        let arr = line.split(" ").collect::<Vec<&str>>();

        let nb: u8 = arr[1].parse().unwrap();
        let from: usize = arr[3].parse().unwrap();
        let to: usize = arr[5].parse().unwrap();

        for i in 0..nb{
            match stacks[from-1].pop(){
                Some(elt) => {stacks[to-1].push(elt);},
                None => {}
            }
        }
        println!("{line:?} {nb} {from} {to}");
        println!("Final score: {stacks:?}");
        // return;

    }
    println!("Final score: {stacks:?}");

    for i in 0..stacks.len(){
        print!("{:?}",stacks[i].pop().unwrap());
    }
    
}

fn main(){
    step1();
    step2();
}