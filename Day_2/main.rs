use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use self::Choice::*;

enum HandResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Choice{
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
pub trait Beats {
    fn beats(&self) -> Self;
    fn beater(&self) -> Self;
}
impl Beats for Choice {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
    fn beater(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Paper => Scissors,
            Scissors => Rock,
            Rock => Paper,
        }
    }
}


fn adv_choice(letter: &str) -> Result<Choice, ()>{

    match letter{
        "A" => { Ok(Rock)},
        "B" => { Ok(Paper)},
        "C" => { Ok(Scissors)},
        _ => { Err(()) }
    }
}

fn my_choice(letter: &str) -> Result<Choice, ()>{

    match letter{
        "X" => { Ok(Rock)},
        "Y" => { Ok(Paper)},
        "Z" => { Ok(Scissors)},
        _ => { Err(()) }
    }
}

fn my_choice2(adv: Choice,res: &str) -> Result<Choice, ()>{
    let (adv_beats, adv_beater) = (adv.beats(), adv.beater());
    println!("{res}");
    match res{
        "X" => { Ok(adv_beats)},
        "Y" => { Ok(adv.clone())},
        "Z" => { Ok(adv_beater)},
        _ => { Err(()) }
    }
    
}

fn result(adv: Choice, moi: Choice) -> HandResult{
    let (own_beats, other_beats) = (moi.beats(), adv.beats());
    match (own_beats, other_beats) {
        _ if own_beats == adv => HandResult::Win,
        _ if other_beats == moi => HandResult::Lose,
        _                            => HandResult::Draw,
    }

}
fn calcul_max(moi: Choice, res: HandResult) -> i32{
    moi as i32 + res as i32
}

fn step1(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    let mut score = 0;

    for line in buf_reader.lines() {
        let moi = my_choice(&line.as_ref().unwrap()[2..3]).unwrap();
        let adv = adv_choice(&line.as_ref().unwrap()[0..1]).unwrap();

        match result(adv, moi){
            HandResult::Win => {println!("Gagné!")},
            HandResult::Draw => {println!("Null!")},
            HandResult::Lose => {println!("Perdu!")}
        }

        score += calcul_max(moi, result(adv, moi));
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
        let adv = adv_choice(&line.as_ref().unwrap()[0..1]).unwrap();
        let moi = my_choice2(adv,&line.as_ref().unwrap()[2..3]).unwrap();

        match result(adv, moi){
            HandResult::Win => {println!("Gagné!")},
            HandResult::Draw => {println!("Null!")},
            HandResult::Lose => {println!("Perdu!")}
        }

        score += calcul_max(moi, result(adv, moi));
    }
    println!("Final score: {score}");
}

fn main(){
    step2();
}