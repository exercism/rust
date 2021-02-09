struct Board {
    pieces: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

impl Board {
    fn annotated(&self) -> Vec<String> {
        (0..self.num_rows).map(|y| self.annotated_row(y)).collect()
    }

    fn annotated_row(&self, y: usize) -> String {
        self.pieces[y]
            .iter()
            .enumerate()
            .map(|(x, &c)| {
                if c == ' ' {
                    self.count_neighbouring_mines_char(x, y)
                } else {
                    c
                }
            })
            .collect::<String>()
    }

    fn count_neighbouring_mines_char(&self, x: usize, y: usize) -> char {
        let mut count = 0;
        for x1 in neighbouring_points(x, self.num_cols) {
            for y1 in neighbouring_points(y, self.num_rows) {
                let piece = self.pieces[y1][x1];
                if piece == '*' {
                    count += 1;
                }
            }
        }
        if count == 0 {
            ' '
        } else {
            (b'0' + count) as char
        }
    }
}

pub fn annotate(pieces: &[&str]) -> Vec<String> {
    if pieces.is_empty() {
        return Vec::new();
    }
    let pieces_vec = pieces.iter().map(|&r| r.chars().collect()).collect();
    Board {
        pieces: pieces_vec,
        num_rows: pieces.len(),
        num_cols: pieces[0].len(),
    }
    .annotated()
}

fn neighbouring_points(x: usize, limit: usize) -> Vec<usize> {
    let mut offsets = vec![x];
    if x >= 1 {
        offsets.push(x - 1);
    }
    if x + 2 <= limit {
        offsets.push(x + 1);
    }
    offsets
}
