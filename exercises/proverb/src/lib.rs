pub fn build_proverb(list: &[&str]) -> String {
    let mut v = Vec::new();
    for index in 0..list.len() {
        if index == list.len() - 1 {
            v.push(format!("And all for the want of a {}.", list[0]));
        } else {
            v.push(format!("For want of a {} the {} was lost.", list[index], list[index+1]));
        }
    }
    v.join("\n")
}
