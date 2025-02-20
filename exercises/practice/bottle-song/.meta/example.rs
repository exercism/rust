pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (1..=take_down)
        .map(|taken_down| {
            let b = start_bottles - taken_down;
            let b_word = number_to_eng_word(b).to_lowercase();
            let b_noun = if b == 1 { "bottle" } else { "bottles" };
            let prev_b = b + 1;
            let prev_b_word = number_to_eng_word(prev_b);
            let prev_b_noun = if prev_b == 1 { "bottle" } else { "bottles" };
            [
                format!("{prev_b_word} green {prev_b_noun} hanging on the wall"),
                format!("{prev_b_word} green {prev_b_noun} hanging on the wall"),
                "And if one green bottle should accidentally fall".into(),
                format!("There'll be {b_word} green {b_noun} hanging on the wall."),
            ]
            .join(",\n")
        })
        .collect::<Vec<String>>()
        .join("\n\n")
        + "\n"
}

fn number_to_eng_word(digit: u32) -> String {
    match digit {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => panic!("Didn't bother adding numbers past 10..."),
    }
    .to_string()
}
