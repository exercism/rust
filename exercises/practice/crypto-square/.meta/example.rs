pub fn encrypt(input: &str) -> String {
    let mut input: Vec<_> = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();
    if input.is_empty() {
        return String::new();
    }

    let width = (input.len() as f64).sqrt().ceil() as usize;
    let size = width * width;

    // skip last row if already empty
    let last_row = if input.len() + width > size {
        width
    } else {
        width - 1
    };

    // padding
    input.resize(size, ' ');

    // prevent input from being moved into closure below
    let input = &input;

    // transpose
    let mut res: String = (0..width)
        .flat_map(|col| {
            (0..last_row)
                .map(move |row| input[row * width + col])
                .chain(std::iter::once(' '))
        })
        .collect();

    // trailing space separator
    res.pop();

    res
}
