//! Example implementation for reverse-string
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect::<String>()
}
