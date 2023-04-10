pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut max_values = vec![vec![0; (max_weight + 1) as usize]; items.len() + 1];

    for i in 1..=items.len() {
        let item_weight = items[i - 1].weight as usize;
        let item_value = items[i - 1].value;

        for w in 0..=(max_weight as usize) {
            if item_weight <= w {
                max_values[i][w] = std::cmp::max(
                    max_values[i - 1][w],
                    max_values[i - 1][w - item_weight] + item_value,
                );
            } else {
                max_values[i][w] = max_values[i - 1][w];
            }
        }
    }

    max_values[items.len()][max_weight as usize]
}
