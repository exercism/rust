#[derive(PartialEq)]
enum Token<'a> {
    Number(i32),
    NonNumber(&'a str),
}

fn apply_op<'a, 'b>(num1: i32, words: &'a [Token<'b>]) -> Option<(i32, &'a [Token<'b>])> {
    let number_pos = words.iter().position(|w| match w {
        Token::Number(_) => true,
        Token::NonNumber(_) => false,
    })?;
    let (op_and_num, mut remainder) = words.split_at(number_pos + 1);
    let (op, num2) = op_and_num.split_at(number_pos);
    let &num2 = match num2 {
        [Token::Number(i)] => i,
        _ => unreachable!(
            "We split at a Number above, so num2 is surely a single-element slice w/ a number"
        ),
    };
    match op {
        [Token::NonNumber("plus")] => Some(num1 + num2),
        [Token::NonNumber("minus")] => Some(num1 - num2),
        [Token::NonNumber("multiplied"), Token::NonNumber("by")] => Some(num1 * num2),
        [Token::NonNumber("divided"), Token::NonNumber("by")] => Some(num1 / num2),
        [Token::NonNumber("raised"), Token::NonNumber("to"), Token::NonNumber("the")] => {
            if Some(&Token::NonNumber("power")) == remainder.first() {
                remainder = remainder.get(1..)?;
                Some(num1.pow(num2 as u32))
            } else {
                None
            }
        }
        _ => None,
    }
    .map(|n| (n, remainder))
}

pub fn answer(c: &str) -> Option<i32> {
    let words = c
        .trim_end_matches('?')
        .split_whitespace()
        .map(|word| {
            if let Ok(i) = word.parse::<i32>() {
                Token::Number(i)
            } else if let Some(i) = word
                .get(..word.len() - 2)
                .and_then(|word| word.parse::<i32>().ok())
            {
                Token::Number(i)
            } else {
                Token::NonNumber(word)
            }
        })
        .collect::<Vec<_>>();
    if words.len() < 3 {
        return None;
    }
    let mut result: i32 = match words[0..3] {
        [Token::NonNumber("What"), Token::NonNumber("is"), Token::Number(i)] => i,
        _ => return None,
    };
    let mut words = words.split_at(3).1;
    while !words.is_empty() {
        let tmp = apply_op(result, words)?;
        result = tmp.0;
        words = tmp.1;
    }
    Some(result)
}
