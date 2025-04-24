pub fn encode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut c_count:i32;
    let mut res = String::new();

    while let Some(c) = chars.next() {
        c_count = 1;
        while let Some(&next_c) = chars.peek() {
            if next_c != c {
                break;
            }
            c_count+=1;
            chars.next();
        }
        if c_count > 1 {
            res.push_str(&c_count.to_string());
        }
        res.push(c);
    }

    res

}

pub fn decode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut res = String::new();
    let mut c_count = 0;

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            c_count = c.to_digit(10).unwrap();
            while let Some(next_c) = chars.peek() {
                if next_c.is_ascii_digit() {
                    c_count = c_count*10 + next_c.to_digit(10).unwrap(); 
                    chars.next();
                }
                else {
                    break;
                }
            }
        }
        else {
            if c_count == 0 {
                c_count = 1;
            }
            res.push_str(&c.to_string().repeat(c_count as usize));
            c_count = 0;
        }
    }
    res
}
