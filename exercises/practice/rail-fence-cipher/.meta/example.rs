pub struct RailFence(u32);

fn uncons(s: &str) -> (&str, &str) {
    s.split_at(s.chars().next().map_or(0, |c| c.len_utf8()))
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    fn next(&self, down: &mut bool, rail: &mut usize) {
        if *down {
            if *rail + 1 < self.0 as usize {
                *rail += 1;
            } else {
                *down = false;
                *rail -= 1;
            }
        } else if *rail > 0 {
            *rail -= 1;
        } else {
            *down = true;
            *rail += 1;
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails =
            vec![String::with_capacity(1 + (text.len() / self.0 as usize)); self.0 as usize];
        let mut down = true;
        let mut rail = 0;

        for ch in text.chars() {
            rails[rail].push(ch);
            self.next(&mut down, &mut rail);
        }

        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_caps = vec![0; self.0 as usize];
        let mut down = true;
        let mut rail = 0;

        for _ in cipher.chars() {
            rail_caps[rail] += 1;
            self.next(&mut down, &mut rail);
        }

        // this vector owns the text of each rail
        let mut rails_own = Vec::with_capacity(self.0 as usize);
        let mut skip = 0;

        for &cap in rail_caps.iter() {
            rails_own.push(
                cipher
                    .chars()
                    .skip(skip)
                    .enumerate()
                    .take_while(|&(i, _)| i < cap)
                    .map(|(_, c)| c)
                    .collect::<String>(),
            );
            skip += cap;
        }

        // this vector holds string slices viewing into rails_own
        let mut rails: Vec<&str> = rails_own.iter().map(|r| r.as_ref()).collect();

        let mut out = String::with_capacity(cipher.len());
        down = true;
        rail = 0;

        while rails.iter().any(|r: &&str| !r.is_empty()) {
            let (head, t_rail) = uncons(rails[rail]);
            rails[rail] = t_rail;
            self.next(&mut down, &mut rail);
            out.push_str(head);
        }

        out
    }
}
