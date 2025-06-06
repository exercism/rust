use std::iter::successors;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {

    let res: Vec<u8> = values
    .iter()
    .flat_map(|&num| {
        
        // Represent num in base 128
        let mut base128: Vec<u8> = successors(Some(num), |&n| {
            if n >= 128 {
                Some(n / 128)
            } else {
                None
            }
        })
        .map(|n| (n % 128) as u8)
        .collect();
        
        // Put it in right order
        base128.reverse();

        // Set MSB to 1 for all bytes but the last
        for i in 0..base128.len().saturating_sub(1) {
            base128[i] += 0x80;
        }

        base128
    })
    .collect();

    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {

    let mut slice = bytes;
    let mut last_of_number: bool = false;
    let mut res: Vec<u32> = Vec::new();
    let mut num: u32 = 0;

    while let Some((&byte, rest)) = slice.split_first() {
        if byte & 0x80 != 0 {
            last_of_number = false;
            num <<= 7;
            num += (byte & 0b0111_1111) as u32;
        }
        else {
            num <<= 7;
            num += byte as u32;
            res.push(num);
            num = 0;
            last_of_number = true;
        }
        slice = rest;
    }

    if !last_of_number {
        return Err(Error::IncompleteNumber);
    }

    Ok(res)

}
