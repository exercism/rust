static ABC: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_diamond(c: char) -> Vec<String>{
    if c == 'A'{
        return vec!(String::from("A"))
    }
    //unimplemented!("Return the vector of strings which represent the diamond with particular char {}", c);
    let mut result: Vec<String> = Vec::new();

    //build first half
     for e in ABC.chars(){
         result.push(get_line(e, c));
         if e == c{
             break;
         }
     }

     //Todo: build second half
     //...
    result
}

fn get_line(e: char, c:char) -> String{
    let mut r = String::new();
    let letter_e = get_letter_line(e);
    let letter_c = get_letter_line(c);
    let ws = letter_c.len() - letter_e.len(); //whitespaces always even

    //left
    for i in 0..ws/2{ r.push(' ');}
    //letter line
    for i in letter_e.chars(){r.push(i)}
    //right
    for i in 0..ws/2{ r.push(' ');}
    r
}

fn get_letter_line(e: char) -> String{
    let mut r = String::new();
    let odd = (0..)
    .filter(|x| x % 2 != 0)
    .take(26)
    .collect::<Vec<usize>>()[ABC.find(e).unwrap()];
    for i in 0..odd{
        if i == 0 || i == odd-1{
            r.push(e);
        } else{
            r.push(' ');
        }
    }
    r
}

