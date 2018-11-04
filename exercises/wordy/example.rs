#[derive(Debug, PartialEq)]
struct WordProblem {
    command: String,
}

#[derive(Debug, PartialEq)]
struct Token {
    value: String,
}

impl Token {
    fn is_valid(&self) -> bool {
        !self.value.is_empty() && (self.is_operand() || self.is_operator())
    }

    fn is_operand(&self) -> bool {
        self.value.chars().all(|c| c.is_numeric() || c == '-')
    }

    fn is_operator(&self) -> bool {
        self.value == String::from("plus")
            || self.value == String::from("minus")
            || self.value == String::from("multiplied")
            || self.value == String::from("divided")
    }
}

pub fn answer(c: &str) -> Option<isize> {
    WordProblem::new(c).answer()
}

impl WordProblem {
    fn new(c: &str) -> Self {
        WordProblem {
            command: String::from(c),
        }
    }

    fn answer(&self) -> Option<isize> {
        let mut t = self.tokens();
        let mut result: isize = 0;
        let mut opr = "plus".to_string();

        if t.len() <= 1 {
            None
        } else {
            while t.len() > 1 {
                result = self.evaluate(result, opr, self.operand(&t.remove(0)));
                opr = self.operator(&t.remove(0));
            }
            result = self.evaluate(result, opr, self.operand(&t.remove(0)));
            Some(result)
        }
    }

    fn evaluate(&self, mut r: isize, operator: String, operand: isize) -> isize {
        if operator == String::from("plus") {
            r += operand;
            r
        } else if operator == String::from("minus") {
            r -= operand;
            r
        } else if operator == String::from("multiplied") {
            r *= operand;
            r
        } else if operator == String::from("divided") {
            r /= operand;
            r
        } else {
            r
        }
    }

    fn operand(&self, t: &Token) -> isize {
        t.value.parse().unwrap()
    }

    fn operator(&self, t: &Token) -> String {
        String::from(t.value.clone())
    }

    fn tokens(&self) -> Vec<Token> {
        self.command
            .split(|c: char| c.is_whitespace() || c == '?')
            .map(|w| Token {
                value: String::from(w),
            })
            .filter(|t| t.is_valid())
            .collect()
    }
}
