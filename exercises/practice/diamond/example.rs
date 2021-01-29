static ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_diamond(diamond_char: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let diamond_char = diamond_char.to_ascii_uppercase();
    if ABC.find(diamond_char).is_none() {
        return result;
    }
    if diamond_char == 'A' {
        return vec![String::from("A")];
    }

    //build first half
    for char_in_abc in ABC.chars() {
        result.push(get_line(char_in_abc, diamond_char).clone());
        if char_in_abc == diamond_char {
            break;
        }
    }

    //build second half
    let mut rev = result.clone();
    rev.pop(); //remove middle piece to avoid duplicates
    for line in rev.drain(..).rev() {
        result.push(line);
    }

    result
}

fn get_line(char_in_abc: char, diamond_char: char) -> String {
    let mut r = String::new();
    let letter_e = get_letter_line(char_in_abc);
    let letter_c = get_letter_line(diamond_char);
    let ws = letter_c.len() - letter_e.len(); //number of whitespaces

    //left
    for _ in 0..ws / 2 {
        r.push(' ');
    }
    //letter line
    for i in letter_e.chars() {
        r.push(i)
    }
    //right
    for _ in 0..ws / 2 {
        r.push(' ');
    }
    r
}

fn get_letter_line(char_in_abc: char) -> String {
    let mut r = String::new();
    let odd = (0..)
        .filter(|x| x % 2 != 0)
        .nth(ABC.find(char_in_abc).unwrap())
        .unwrap();
    for i in 0..odd {
        if i == 0 || i == odd - 1 {
            r.push(char_in_abc);
        } else {
            r.push(' ');
        }
    }
    r
}
