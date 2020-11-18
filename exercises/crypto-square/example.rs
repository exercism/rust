extern crate itertools;
use itertools::Itertools;

/// Encrypt the input string using square cryptography
pub fn encrypt(input: &str) -> String {
    let prepared = prepare(input);
    if prepared.is_empty() {
        return String::new();
    }

    let (cols, rows) = dimensions(prepared.len());

    let mut output = String::with_capacity(input.len());
    for chunk_iterator in SquareIndexer::new(rows, cols).chunks(cols).into_iter() {
        for ch_idx in chunk_iterator {
            if ch_idx < prepared.len() {
                output.push(prepared[ch_idx]);
            }
        }
        output.push(' ');
    }

    // we know there's one extra space at the end
    output.pop();

    output
}

/// Construct a vector of characters from the given input.
///
/// Constrain it to the allowed chars: lowercase ascii letters.
/// We construct a vector here because the length of the input
/// matters when constructing the output, so we need to know
/// how many input chars there are. We could treat it as a stream
/// and just stream twice, but collecting it into a vector works
/// equally well and might be a bit faster.
fn prepare(input: &str) -> Vec<char> {
    let mut output = Vec::with_capacity(input.len());

    output.extend(
        input
            .chars()
            .filter(|&c| c.is_ascii() && !c.is_whitespace() && !c.is_ascii_punctuation())
            .map(|c| c.to_ascii_lowercase()),
    );

    // add space padding to the end such that the actual string returned
    // forms a perfect rectangle
    let (r, c) = dimensions(output.len());
    output.resize(r * c, ' ');

    output.shrink_to_fit();

    output
}

/// Get the dimensions of the appropriate bounding rectangle for this encryption
///
/// To find `(rows, cols)` such that `cols >= rows && cols - rows <= 1`, we find
/// the least square greater than or equal to the message length. Its square root
/// is the cols. If the message length is a perfect square, `rows` is the same.
/// Otherwise, it is one less.
fn dimensions(length: usize) -> (usize, usize) {
    let cols = (length as f64).sqrt().ceil() as usize;
    let rows = if cols * cols == length {
        cols
    } else {
        cols - 1
    };
    (rows, cols)
}

/// Iterator over the indices of the appropriate chars of the output.
///
/// For a (2, 3) (r, c) grid, yields (0, 3, 1, 4, 2, 5).
/// Does no bounds checking or space insertion: that's handled elsewhere.
#[derive(Debug)]
struct SquareIndexer {
    rows: usize,
    cols: usize,
    cur_row: usize,
    cur_col: usize,
    max_value: usize,
}

impl SquareIndexer {
    fn new(rows: usize, cols: usize) -> SquareIndexer {
        SquareIndexer {
            rows,
            cols,
            cur_row: 0,
            cur_col: 0,
            max_value: rows * cols,
        }
    }
}

impl Iterator for SquareIndexer {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let value = self.cur_row + (self.cur_col * self.rows);
        let output = if value < self.max_value && self.cur_row < self.rows {
            Some(value)
        } else {
            None
        };

        // now increment internal state to next value
        self.cur_col += 1;
        if self.cur_col >= self.cols {
            self.cur_col = 0;
            self.cur_row += 1;
        }

        output
    }
}
