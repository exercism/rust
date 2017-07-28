pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut out: Vec<String> = vec![];
    for i in 1..list.len() {
        out.push(format!("For want of a {} the {} was lost.", list[i-1], list[i]));
    }
    let end: String;
    if list.len() > 2 {
        end = format!("{}{} {}", list[2], list[1], list[0]);
    } else {
        end = format!("{}", list[0]);
    }
    out.push(format!("And all for the want of a {}.", end));
    out.join("\n")
}
