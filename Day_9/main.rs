use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        (((self.x - p.x).pow(2) + (self.y - p.y).pow(2)) as f64).sqrt()
    }
    fn dx(&self, p: &Point) -> i32 {
        self.x - p.x
    }
    fn dy(&self, p: &Point) -> i32 {
        self.y - p.y
    }
    fn change_coor(&mut self, x: i32, y: i32){
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug)]
struct Game{
    head: Point,
    tail: Point,
    board: Vec<Vec<bool>>
}

impl Game{
    fn new() -> Game {
        let mut g = Game{
            head: Point{x: 2500,y: 2500},
            tail: Point{x: 2500,y: 2500},
            board: vec![vec![false; 5000]; 5000]
        };
        g.board[0][0] = true;
        g
    }

    fn size(&self) -> (usize, usize) {
        (self.board.len(), self.board[0].len())
    }

    fn need_move(&self) -> bool {
        if self.head.distance(&self.tail) >= 2.0 {
            //println!("Need move");
            return true;
        }
        false
    }

    fn parse_move(&mut self, m: &str) -> bool{
        let direction = m.split(" ").collect::<Vec<&str>>()[0];
        let nb_iter = m.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        for _ in 0..nb_iter {
            
            match direction {
                "R" => {
                    self.move_head(self.head.x as usize, (self.head.y + 1) as usize);
                },
                "L" => {
                    self.move_head(self.head.x as usize, (self.head.y - 1) as usize);
                },
                "U" => {
                    self.move_head((self.head.x + 1) as usize, self.head.y as usize);
                },
                "D" => {
                    self.move_head((self.head.x - 1) as usize, self.head.y as usize);
                },
                _ => {println!("Unknown direction");}
            }
            //println!("{:?} {:?}",self.head, self.tail);
        }
        true
    }

    fn move_head(&mut self, x: usize, y: usize){
        if x >= self.size().0 || y >= self.size().1{
            println!("Need bigger array");
            process::exit(1);
        }
        if self.head.distance(&Point{x: x as i32, y: y as i32}) >= 2.0{
            println!("Illegal move");
            process::exit(1);
        }
        self.head.change_coor(x as i32, y as i32);

        if self.need_move() {
            self.move_tail();
        }
    }

    fn move_tail(&mut self){
        if self.head.distance(&self.tail) == 2.0{
            //println!("Droit {} {}",if self.head.dx(&self.tail) == 2 {1} else {0}, if self.head.dy(&self.tail) == 2 {1} else {0});
            self.tail.change_coor(
                self.tail.x + if self.head.dx(&self.tail).abs() == 2 {self.head.dx(&self.tail)/2} else {0},
                self.tail.y + if self.head.dy(&self.tail).abs() == 2 {self.head.dy(&self.tail)/2} else {0}
            )
        }else if self.head.distance(&self.tail) >= 2.0 {
            //println!("Diag");
            self.tail.change_coor(
                self.tail.x + if self.head.dx(&self.tail).abs() == 2 {self.head.dx(&self.tail)/2} else {self.head.dx(&self.tail)},
                self.tail.y + if self.head.dy(&self.tail).abs() == 2 {self.head.dy(&self.tail)/2} else {self.head.dy(&self.tail)}
            )
        }else{
            println!("no need move");
            process::exit(1);
        }
        self.board[self.tail.x as usize][self.tail.y as usize] = true;
    }

    fn score(&self) -> usize{
        self.board.iter().flatten().filter(|e| e == &&true).count()
    }

}

// impl std::fmt::Display for Game {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let mut affiche = String::from("")
//         for i in self.
//         write!(f, "(value a: {}, value b: {})", self.a, self.b)
//     }
// }

fn step1(){
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);

    

    let mut game = Game::new();
    

    for line in buf_reader.lines() {
        let line = line.unwrap();
        println!("{}",line);
        game.parse_move(&line);
        
    }


    
    println!("Nb cases: {}", game.score());

    // for i in 0..6{
    //     println!("{:?}",game.board[i]);
    // }
}

fn main(){
    step1();
    //step2();
}