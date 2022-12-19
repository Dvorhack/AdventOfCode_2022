fn add(a: u32, b: Value) -> u32 {
    match b {
        Value::Old => {return a + a;},
        Value::Num(c) => {return a + c;}
    }
}
fn mul(a: u32, b: Value) -> u32 {
    match b {
        Value::Old => {return a * a;},
        Value::Num(c) => {return a * c;}
    }
}

#[derive(Debug)]
enum Value{
    Old,
    Num(u32)
}

#[derive(Debug)]
struct Monkey{
    inspected_items: usize,
    items: Vec<u32>,
    operation: fn(u32, Value) -> u32,
    operator_number: Value,
    divisor: u32,
    true_action: u32,
    false_action: u32
}

impl Monkey{
    fn parse(block: &str) -> Option<Monkey> {
        // Remove first line
        let mut lines = block.lines().skip(1);

        // Get items
        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
            .split_terminator(", ")
            .filter_map(|s| s.parse().ok())
            .collect();
        
        //Get operation and operand
        let (_, operation) = lines.next()?.split_once("= old ")?;
        let (operator, operand) = operation.split_once(" ")?;
        let operand = match operand {
            "old" => Value::Old,
            _ => {
                let n = operand.parse().ok()?;
                Value::Num(n)
            }
        };
        let operation = match operator {
            "+" => add,
            "*" => mul,
            _ => panic!("Inalid input"),
        };

        // Get divisor
        let (_, divisor) = lines.next()?.split_once("divisible by ")?;
        let divisor: u32 = divisor.parse::<u32>().unwrap();

        // Ger true action
        let (_, true_action) = lines.next()?.split_once("to monkey ")?;
        let true_action: u32 = true_action.parse::<u32>().unwrap();

        // Ger false action
        let (_, false_action) = lines.next()?.split_once("to monkey ")?;
        let false_action: u32 = false_action.parse::<u32>().unwrap();

        Some(Monkey{
            inspected_items: 0,
            items: items,
            operation: operation,
            operator_number: operand,
            divisor: divisor,
            true_action: true_action,
            false_action: false_action
        })
    }
}

fn step1() -> Result<(),()>{
    let input = std::fs::read_to_string("input_test.txt").unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for block in input.split("\n\n") {
        monkeys.push(Monkey::parse(block).ok_or("Problem parsing").unwrap());
    }
    for monk in monkeys{
        println!("{monk:?}");
    }

    for idx in 0..monkeys.len() {
        let monkey = monkeys[idx];
        for item in monkey.items {
            // remove item from inventory
            // monkey inspects item
            // monkey does operation
            // you are relieved
            // monkey tests worry level
            // monkey throws item to other monkey
        }
    }

    Ok(())
}


fn main(){
    step1();
}