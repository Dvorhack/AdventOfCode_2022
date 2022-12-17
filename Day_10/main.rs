struct CPU{
    x: isize,
    tick: usize,
    signal_trigger: [usize; 6],
    strength: usize,
    current_row: [bool; 40]
}

enum OPCODE {
    NOP,
    ADD(isize)
}

impl CPU {
    fn new() -> CPU{
        CPU{
            x: 1,
            tick: 0,
            signal_trigger: [20, 60, 100, 140, 180, 220],
            strength: 0,
            current_row: [false; 40]
        }
    }
    fn add_tick(&mut self){
        if (self.x-1..self.x+2).contains(&(self.tick as isize % 40)){
            self.current_row[(self.tick % 40)] = true;
        }
        self.tick += 1;
        self.signal_strengths();
        if self.tick % 40 == 0{
            self.print_line();
        }
    }
    fn print_line(&mut self){
        for i in self.current_row{
            if i{
                print!("X");
            }else{
                print!(".");
            }
        }
        println!("");
        self.current_row = [false; 40];
    }
    fn execute(&mut self, op: OPCODE){
        match op {
            OPCODE::NOP => {
                self.add_tick();
            },
            OPCODE::ADD(y) => {
                self.add_tick();
                self.add_tick();
                self.x += y;
            },
            _ => {
                panic!("Unknown opcode")
            }
        }
        
    }
    fn signal_strengths(&mut self) {
        if self.signal_trigger.contains(&self.tick){
            //println!("{} {}",self.tick, self.x);
            self.strength += self.tick * self.x as usize;
        }
    }
}

fn step1(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cpu = CPU::new();

    for line in input.lines(){
        match &line[..4]{
            "noop" => cpu.execute(OPCODE::NOP),
            "addx" => cpu.execute(OPCODE::ADD(line[5..].parse::<isize>().unwrap())),
            _ => panic!("Unknwn opcode {}",&line[..4])
        }

    }
    println!("Strength: {}",cpu.strength);
}

fn step2(){
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cpu = CPU::new();

    for line in input.lines(){
        match &line[..4]{
            "noop" => cpu.execute(OPCODE::NOP),
            "addx" => cpu.execute(OPCODE::ADD(line[5..].parse::<isize>().unwrap())),
            _ => panic!("Unknwn opcode {}",&line[..4])
        }

    }
    println!("Strength: {}",cpu.strength);
}

fn main(){
    step1();
}