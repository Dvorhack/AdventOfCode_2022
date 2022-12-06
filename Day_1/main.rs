use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {

    let file_path = "input.txt";

    let file = File::open(file_path)
  .expect("file not found!");
 let  buf_reader = BufReader::new(file);

    let mut total = 0;
    let mut max_cal = HashMap::new();
    max_cal.insert(1,0);
    max_cal.insert(2,0);
    max_cal.insert(3,0);

 for line in buf_reader.lines() {
  match line.unwrap().parse::<i32>(){
    Ok(nb)=> {total += nb},
    Err(_)=>{
        if total > *max_cal.get(&3).unwrap(){
            if total > *max_cal.get(&2).unwrap() {
                if total > *max_cal.get(&1).unwrap(){
                    *max_cal.get_mut(&3).unwrap() = *max_cal.get(&2).unwrap();
                    *max_cal.get_mut(&2).unwrap() = *max_cal.get(&1).unwrap();
                    *max_cal.get_mut(&1).unwrap() = total;
                }else{
                    *max_cal.get_mut(&3).unwrap() = *max_cal.get(&2).unwrap();
                    *max_cal.get_mut(&2).unwrap() = total;
                }
            }else{
                *max_cal.get_mut(&3).unwrap() = total;
            }
        }
        total = 0;
    }
  }
 }
 let somme: i32 = max_cal.values().sum();
 
 println!("Top 3: {:?}\nSomme: {}",max_cal,somme);
    
}