use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};


fn step1() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);

    let nb_columns = buf_reader.lines()
                                .nth(1)
                                .expect("File not 5 line long")
                                .expect("Cannot get line 5")
                                .len();

    let file = File::open(file_path)
            .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    let nb_lines = buf_reader.lines().count();

    println!("Nb columns {nb_columns} Nb lines {nb_lines}");
    let mut map = vec![vec![0; nb_columns]; nb_lines];

    println!("{:?}",map);
    
    let file = File::open(file_path)
            .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    // Create the 2D map
    for (i,line) in buf_reader.lines().enumerate() {
        let line = line.unwrap();

        for (j,c) in line.chars().enumerate(){
            print!("{c}");
            map[i][j] = (c as u8) - 0x30;
        }
        println!("");
       
    }
    println!("{:?}",map);
    let mut visible = nb_columns*2 + nb_lines * 2 - 4;
    // Find the visible trees
    for i in 1..nb_lines-1{
        for j in 1..nb_columns-1{
            println!("{}",map[i][j]);
            // Check visible left
            if &map[i][..j].iter().max().unwrap() < &&map[i][j]{
                println!("Left");
                visible += 1;
                continue;

            // Check visible right
            }else if &map[i][j+1..].iter().max().unwrap() < &&map[i][j]{
                println!("Right");
                visible += 1;
                continue;

            // Check visible top
            }else if map.clone().into_iter().flatten().skip(j).step_by(nb_columns).take(i).max().unwrap() < map[i][j]{
                println!("Top");
                visible += 1;
                continue;
            // Check visible bottom
            }else if map.clone().into_iter().flatten().skip(j).step_by(nb_columns).skip(i+1).max().unwrap() < map[i][j]{
                println!("Top");
                visible += 1;
                continue;
            }
            // println!("toto {:?}",map.clone().into_iter().flatten().skip(j).step_by(nb_columns).skip(i+1).collect::<Vec<u8>>());
        }
    }
    println!("visible {visible}");
}

fn step2() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);

    let nb_columns = buf_reader.lines()
                                .nth(1)
                                .expect("File not 5 line long")
                                .expect("Cannot get line 5")
                                .len();

    let file = File::open(file_path)
            .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    let nb_lines = buf_reader.lines().count();

    println!("Nb columns {nb_columns} Nb lines {nb_lines}");
    let mut map = vec![vec![0; nb_columns]; nb_lines];

    println!("{:?}",map);
    
    let file = File::open(file_path)
            .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    // Create the 2D map
    for (i,line) in buf_reader.lines().enumerate() {
        let line = line.unwrap();

        for (j,c) in line.chars().enumerate(){
            print!("{c}");
            map[i][j] = (c as u8) - 0x30;
        }
        println!("");
       
    }
    println!("{:?}",map);
    let mut best_score = 0;
    // Find the visible trees
    for i in 1..nb_lines-1{
        for j in 1..nb_columns-1{
            let current = map[i][j];
            let mut local_score = 1;
            println!("{current}");
            // left
            local_score *= &map[i][..j].iter().rev().fold_while(0,|acc, e| {
                if e < &current {
                    Continue(acc + 1)
                }else if e == &current {
                    Done(acc + 1)
                }else{
                    Done(acc + 1)
                }
            }).into_inner();
            // println!("local {local_score}");

            //right
            local_score *= &map[i][j+1..].iter().fold_while(0,|acc, e| {
                if e < &current {
                    Continue(acc + 1)
                }else if e == &current {
                    Done(acc + 1)
                }else{
                    Done(acc + 1)
                }
            }).into_inner();
            // println!("local {local_score}");

            //top
            local_score *= map.clone().into_iter().flatten().skip(j).step_by(nb_columns).take(i).collect::<Vec<_>>().into_iter()
            .rev().fold_while(0,|acc, e| {
                if e < current {
                    Continue(acc + 1)
                }else if e == current {
                    Done(acc + 1)
                }else{
                    Done(acc + 1)
                }
            }).into_inner();
            // println!("local {local_score}");

            //bottom
            local_score *= map.clone()
                            .into_iter()
                            .flatten()
                            .skip(j)
                            .step_by(nb_columns)
                            .skip(i+1)
                            .fold_while(0,|acc, e| {
                                if e < current {
                                    Continue(acc + 1)
                                }else if e == current {
                                    Done(acc + 1)
                                }else{
                                    Done(acc + 1)
                                }
                            }).into_inner();

            println!("local {local_score}");

            // println!("toto {:?}",&map[i][..j].iter().rev().fold_while(0,|acc, e| {
            //     if e < &current {
            //         Continue(acc + 1)
            //     }else if e == &current {
            //         Done(acc + 1)
            //     }else{
            //         Done(acc)
            //     }
            // }).into_inner());
            if local_score > best_score{
                best_score = local_score;
            }
        }
    }
    println!("best_score {best_score}");
}

fn main(){
    //step1();
    step2();
}