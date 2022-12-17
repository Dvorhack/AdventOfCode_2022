
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}


fn step1(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let start = Point { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;

    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.lines(){
        // Get the elements form the line
        let (dir, num) = line.split_once(' ').unwrap();
        let num = num.parse::<i32>().unwrap();

        for _ in 0..num{
            // Move the head
            match dir {
                "U" => head.y += 1,
                "L" => head.x += 1,
                "R" => head.x -= 1,
                "D" => head.y -= 1,
                _ => panic!("Unvalid position {}",dir)
            }

            // Check if tail needs to move
            let diff = Point {
                x: head.x - tail.x,
                y: head.y - tail.y
            };
            if diff.x.abs() > 1 || diff.y.abs() > 1 {
                // Add the sign of the difference
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                visited.insert(tail);
            }
        }
        
    }

    println!("Visited {} points", visited.len());

}

fn step2(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let start = Point { x: 0, y: 0 };
    let snake_len = 10;
    let target_node = 9;
    let mut snake = vec![start;snake_len];

    let mut visited = HashSet::new();
    visited.insert(snake[target_node]);

    for line in input.lines(){
        // Get the elements form the line
        let (dir, num) = line.split_once(' ').unwrap();
        let num = num.parse::<i32>().unwrap();

        for _ in 0..num{
            // Move the head
            match dir {
                "U" => snake[0].y += 1,
                "L" => snake[0].x += 1,
                "R" => snake[0].x -= 1,
                "D" => snake[0].y -= 1,
                _ => panic!("Unvalid position {}",dir)
            }
            for i in 1..snake_len{
                // Check if tail needs to move
                let diff = Point {
                    x: snake[i-1].x - snake[i].x,
                    y: snake[i-1].y - snake[i].y
                };
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    // Add the sign of the difference
                    snake[i].x += diff.x.signum();
                    snake[i].y += diff.y.signum();
                }
            }
            visited.insert(snake[target_node]);
        }
        
    }

    println!("Visited {} points", visited.len());

}

fn main(){
    // step1();
    step2();
}