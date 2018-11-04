#[derive(Debug, PartialEq)]
struct Token<'a> {
    value: &'a str,
}

impl <'a> Token<'a> {
    fn is_valid(&self) -> bool {
        !self.value.is_empty() && (self.is_operand() || self.is_operator())
    }

    fn is_operand(&self) -> bool {
        self.value.chars().all(|c| c.is_numeric() || c == '-')
    }

    fn is_operator(&self) -> bool {
        self.value == "plus"
            || self.value == "minus"
            || self.value == "multiplied"
            || self.value == "divided"
    }
}

pub fn answer(c: &str) -> Option<isize> {
    let mut t = tokens(c);
    let mut result: isize = 0;
    let mut opr = "plus";

    if t.len() <= 1 {
        None
    } else {
        while t.len() > 1 {
            result = evaluate(result, opr, operand(&t.remove(0)));
            opr = operator(&t.remove(0));
        }
        result = evaluate(result, opr, operand(&t.remove(0)));
        Some(result)
    }
}

fn evaluate(r: isize, operator: &str, operand: isize) -> isize {
    match operator {
        "plus" => r + operand,
        "minus" => r - operand,
        "multiplied" => r * operand,
        "divided" => r / operand,
        _ => r,
    }
}

fn operand(t: &Token) -> isize {
    t.value.parse().unwrap()
}

fn operator<'a>(t: &Token<'a>) -> &'a str {
    t.value
}

fn tokens<'a>(command: &'a str) -> Vec<Token<'a>> {
    command
        .split(|c: char| c.is_whitespace() || c == '?')
        .map(|w| Token {
            value: w,
        })
        .filter(|t| t.is_valid())
        .collect()
}
