use std::cmp;

pub fn encode(input: &str) -> String {
    input
        .chars()
        .fold(
            (String::new(), ' ', 0, 1),
            |(mut acc, last, last_n, pos), c| {
                // acc = where answer is accumulated
                // last = last character read
                // last_n = accum count for last
                if c == last {
                    if pos == input.len() {
                        // end of string
                        acc += (last_n + 1).to_string().as_str();
                        acc.push(c);
                    }
                    (acc, last, last_n + 1, pos + 1)
                } else {
                    if last_n > 1 {
                        acc += last_n.to_string().as_str();
                    }
                    if last_n > 0 {
                        // ignore initial last (single whitespace)
                        acc.push(last);
                    }
                    if pos == input.len() {
                        // end of string
                        acc.push(c);
                    }
                    (acc, c, 1, pos + 1)
                }
            },
        )
        .0
}

pub fn decode(input: &str) -> String {
    input
        .chars()
        .fold((String::new(), 0), |(mut acc, last_n), c| {
            if let Some(d) = c.to_digit(10) {
                (acc, 10 * last_n + d)
            } else {
                acc += c.to_string().repeat(cmp::max(last_n, 1) as usize).as_str();
                (acc, 0)
            }
        })
        .0
}
