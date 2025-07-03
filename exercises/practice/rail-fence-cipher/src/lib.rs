pub struct RailFence{
    nb_rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { 
            nb_rails: rails as usize, 
            } 
        }

    pub fn encode(&self, text: &str) -> String {
        // Create rails here (since &self is not mutable)
        let mut rails = Vec::new();
        (0..self.nb_rails).for_each(|_| rails.push(Vec::new()));

        // Utils for parse rails in the proper order
        let mut i = 0;
        let mut direction = 1; 

        let chars = text.chars(); 
        // As long as there are some char left, distribute them into rails
        // while let Some(c) = chars.next() {
        for c in chars {
            if i == self.nb_rails - 1 {
                direction = -1;
            }
            else if i == 0 {
                direction = 1;
            }

            rails.get_mut(i).unwrap().push(c); 

            // Update i 
            i = (i as i32 + direction) as usize;
        }

        // Flatten everything
        rails
        .iter()
        .flatten()
        .collect()

    }

    pub fn decode(&self, cipher: &str) -> String {

        let nb_chars = cipher.chars().count();
        let nb_cycles = nb_chars / (2*self.nb_rails-2);
        let nb_left_chars = nb_chars % (2*self.nb_rails-2); 

        // For each cycle : 1 letter in the first rail, 1 letter in the last rail, 2 letters in other rails 
        let mut rails_sizes: Vec<usize> = (0..self.nb_rails).map(|i| {
            if i == 0 || i == self.nb_rails - 1 {
                nb_cycles
            }
            else {
                2*nb_cycles
            }
        })
        .collect();

        println!("nb_chars {nb_chars}, nb_cycles {nb_cycles}, nb_left_chars {nb_left_chars}, {rails_sizes:?}");

        // distributed what is left
        let mut i = 0;
        let mut direction = 1; 
        (0..nb_left_chars).for_each(|_| {
            *rails_sizes.get_mut(i).unwrap() += 1;
            
            if i == self.nb_rails - 1 {
                direction = -1;
            }
            else if i == 0 {
                direction = 1;
            }

            i = (i as i32 + direction) as usize;
        });

        println!("{rails_sizes:?}");

        // Create rails
        let mut rails: Vec<Vec<char>> = Vec::new();
        (0..self.nb_rails).for_each(|_| rails.push(Vec::new()));

        rails_sizes
        .iter()
        .enumerate()
        .fold(0, |char_offset, (rail_index,rail_len)| {
            let rail = rails.get_mut(rail_index).unwrap();
            *rail = cipher.chars().skip(char_offset).take(*rail_len).collect::<Vec<char>>();
            char_offset + *rail_len
        });

        println!("{rails:?}");

        // Build output from rails
        let mut res: Vec<char> = Vec::new();

        i = 0;
        direction = 1; 
        (0..nb_chars).for_each(|_| {
            // Take the first char of the current rail
            res.push(rails.get_mut(i).unwrap().remove(0));

            if i == self.nb_rails - 1 {
                direction = -1;
            }
            else if i == 0 {
                direction = 1;
            }
            i = (i as i32 + direction) as usize;  
        });

        res.iter().collect()

    }
}
