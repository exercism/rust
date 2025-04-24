use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res: BTreeMap<char, i32> = BTreeMap::new();

    for (score, chars) in h {
        for c in chars {
            res.insert(c.to_lowercase().next().unwrap(), *score);
        }
    }

    res

}
