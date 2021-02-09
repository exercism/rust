pub fn build_proverb(items: &[&str]) -> String {
    let mut stanzas = Vec::with_capacity(items.len());
    for index in 0..items.len() {
        if index == items.len() - 1 {
            stanzas.push(format!("And all for the want of a {}.", items[0]));
        } else {
            stanzas.push(format!(
                "For want of a {} the {} was lost.",
                items[index],
                items[index + 1]
            ));
        }
    }
    stanzas.join("\n")
}
