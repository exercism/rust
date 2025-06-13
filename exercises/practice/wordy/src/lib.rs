use regex::Regex;

pub fn answer(command: &str) -> Option<i32> {
    // let mut command = command;

    // Check it starts correctly
    // if command.starts_with("What is") {
    //     command = command["What is".len()..].trim_start();
    // }
    // else {
    //     return None;
    // }

    let command = command.strip_prefix("What is ")?.trim_start();

    // Capture the first number
    let re_first_number = Regex::new(r"^(-?\d+)").ok()?;
    let caps = re_first_number.captures(command)?;
    let first_str = caps.get(1)?.as_str();
    let mut first: i32 = first_str.parse().ok()?;

    // Update command so only the remaining task is left
    let mut command = command[first_str.len()..].trim_start();

    // Match operations : exponential is treated separately
    let re_op = Regex::new(r"(?x)
        ^
        (?P<op1>plus|minus|multiplied\sby|divided\sby)\s+(?P<rhs1>-?\d+)
        |
        raised\sto\sthe\s+(?P<rhs2>-?\d+)(st|nd|rd|th)\s+power
    ").ok()?;

    // Handle the rest of the operations
    while let Some(op_caps) = re_op.captures(command) {
        if let Some(op) = op_caps.name("op1") {
            let rhs: i32 = op_caps.name("rhs1")?.as_str().parse().ok()?;
            match op.as_str() {
                "plus" => first += rhs,
                "minus" => first -= rhs,
                "multiplied by" => first *= rhs,
                "divided by" => first /= rhs,
                _ => {
                    return None;
                }
            }
        }
        else if let Some(rh2) = op_caps.name("rhs2") {
            let exp: u32 = rh2.as_str().parse().ok()?;
            first = first.pow(exp);
        }

        // Update command so only the remaining task is left
        command = command[op_caps.get(0)?.as_str().len()..].trim_start();
    }

    // Make sure the whole thing have been dealt correctly, which means only '?' is left 
    if command != "?" {
        return None;
    }

    Some(first)
}
