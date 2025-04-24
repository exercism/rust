use regex::Regex;

fn starts_with_vowel_or_special(s: &str) -> String {
    let re = Regex::new(r"^(?:[aeiou]|xr|yt)").unwrap();
    if re.is_match(s) {
        format!("{}{}", s, "ay")
    }
    else {
        s.to_string()
    }
}

fn move_leading_consonants_to_end(s: &str) -> String {
    // This regex captures:
    //  - group 1: leading consonants (anything not starting with a vowel)
    //  - group 2: the rest of the string
    let re = Regex::new(r"^([^aeiou]+)(.*)").unwrap();

    if let Some(caps) = re.captures(s) {
        let consonants = caps.get(1).unwrap().as_str();
        let rest = caps.get(2).unwrap().as_str();
        format!("{}{}{}", rest, consonants, "ay")
    }
    else {
        s.to_string()
    }
}

fn move_leading_consonants_and_qu_to_end(s: &str) -> String {
    // This regex captures:
    //  - group 1: leading consonants (anything not starting with a vowel). * means it is zero or more
    //  - group 2: qu
    //  - group 3: the rest of the string
    let re = Regex::new(r"^([^aeiou]*)(qu)(.*)").unwrap();

    if let Some(caps) = re.captures(s) {
        let leading_consonants = caps.get(1).unwrap().as_str();
        let qu = caps.get(2).unwrap().as_str();
        let rest = caps.get(3).unwrap().as_str();
        println!("{} {} {}", leading_consonants, qu, rest);
        format!("{}{}{}{}", rest, leading_consonants, qu, "ay")
    }
    else {
        s.to_string()
    }

}

fn move_leading_consonants_and_y_to_end(s: &str) -> String {
    // This regex captures:
    //  - group 1: leading consonants (anything not starting with a vowel). + means it has to be at least one
    //  - group 2: y
    //  - group 3: the rest of the string
    let re = Regex::new(r"^([^aeiou]+)(y)(.*)").unwrap();

    if let Some(caps) = re.captures(s) {
        let leading_consonants = caps.get(1).unwrap().as_str();
        let y = caps.get(2).unwrap().as_str();
        let rest = caps.get(3).unwrap().as_str();
        println!("{} {} {}", leading_consonants, y, rest);
        format!("{}{}{}{}", y, rest, leading_consonants, "ay")
    }
    else {
        s.to_string()
    }

}

pub fn translate(input: &str) -> String {
    let mut res = String::new();
    // let mut res_inner = String::new();

    for word in input.split_whitespace() {

        let mut res_inner = starts_with_vowel_or_special(word);
        if res_inner != word {
            res_inner.push(' ');
            res.push_str(&res_inner);
            continue;
        }

        res_inner = move_leading_consonants_and_qu_to_end(word);
        println!("{}", res);
        if res_inner != word {
            res_inner.push(' ');
            res.push_str(&res_inner);
            continue;
        }

        res_inner = move_leading_consonants_and_y_to_end(word);
        println!("{}", res);
        if res_inner != word {
            res_inner.push(' ');
            res.push_str(&res_inner);
            continue;
        }

        res_inner = move_leading_consonants_to_end(word);
        println!("{}", res);
        if res_inner != word {
            res_inner.push(' ');
            res.push_str(&res_inner);
            continue;
        }
    }
    res.trim().to_string()
}
