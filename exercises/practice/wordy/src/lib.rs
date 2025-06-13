use regex::Regex;

pub fn answer(command: &str) -> Option<i32> {
    let mut command = command;

    // Check it starts correctly
    if command.starts_with("What is") {
        command = command["What is".len()..].trim_start();
    }
    else {
        return None;
    }

    // Capture the first number
    let re_first_number = Regex::new(r"^(-?\d+)").ok()?;
    let caps = re_first_number.captures(command)?;
    let first_str = caps.get(1)?.as_str();
    let mut first: i32 = first_str.parse().ok()?;

    println!("{first}");
    println!("{command}");
    command = command[first_str.len()..].trim_start();
    println!("{command}");

    // Match operations
    let re_op = Regex::new(r"^(plus|minus|multiplied by|divided by)\s+(-?\d+)").ok()?;

    while let Some(op_caps) = re_op.captures(command) {
        println!("BLLLLLLLLLLLLLA");
        println!("{:?}", op_caps); 
        let bla = op_caps.get(0);
        println!("bla {:?}", bla);
        let bla = op_caps.get(1);
        println!("bla {:?}", bla);
        let second: i32 = op_caps.get(2)?.as_str().parse().ok()?;
        println!("second {second}");
        match op_caps.get(1)?.as_str() {
            "plus" => {
                println!("In plus arm");
                first += second;
            },
            "minus" => {
                println!("In minus arm");
                first -= second;
            },
            "multiplied by" => {
                first *= second;
            },
            "divided by" => {
                first /= second;
            },
            _ => {
                println!("In otherwise arm");
                return None;
            }
        }
        println!("Before update {command}");
        command = command[op_caps.get(0)?.as_str().len()..].trim_start();
        println!("After update {command}");
    }

    println!("Before returning: {command}");
    
    if command != "?" {
        return None;
    
    Some(first)
}
