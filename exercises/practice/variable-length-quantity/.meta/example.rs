#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];

    for value in values {
        res.append(&mut to_bytes_single(*value));
    }
    res
}

fn to_bytes_single(mut value: u32) -> Vec<u8> {
    // over allocates, but avoids growth
    let mut res = Vec::with_capacity(4);

    // 0 must be handled specially, because we need to push one byte
    if value == 0 {
        return vec![0];
    }

    while value > 0 {
        // take the lower 7 bits
        let mut tmp = (value & 0x7f) as u8;
        // remove them from the original value
        value >>= 7;

        // set continuation bit
        if !res.is_empty() {
            tmp |= 0x80;
        }

        res.push(tmp);
    }

    // order is wrong due to the way we pushed the data onto it
    res.reverse();
    res
}

// Alternative solution with hardcoded borders
// /// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
// pub fn to_bytes(values: &[u32]) -> Vec<u8> {
//     let mut res = vec![];
//
//     for &value in values {
//         if value <= 0x7f {
//             res.push(value as u8);
//         } else if value <= 0x3fff {
//             res.push(((value >> 7) & 0xff) as u8 | 0x80);
//             res.push((value & 0x7f) as u8);
//         } else if value <= 0x1f_ffff {
//             res.push(((value >> 14) & 0xff) as u8 | 0x80);
//             res.push(((value >> 7) & 0xff) as u8 | 0x80);
//             res.push((value & 0x7f) as u8);
//         } else if value <= 0x0fff_ffff {
//             res.push(((value >> 21) & 0xff) as u8 | 0x80);
//             res.push(((value >> 14) & 0xff) as u8 | 0x80);
//             res.push(((value >> 7) & 0xff) as u8 | 0x80);
//             res.push((value & 0x7f) as u8);
//         } else {
//             res.push(((value >> 28) & 0xff) as u8 | 0x80);
//             res.push(((value >> 21) & 0xff) as u8 | 0x80);
//             res.push(((value >> 14) & 0xff) as u8 | 0x80);
//             res.push(((value >> 7) & 0xff) as u8 | 0x80);
//             res.push((value & 0x7f) as u8);
//         }
//     }
//     res
// }

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = vec![];
    let mut tmp = 0;
    for (i, b) in bytes.iter().enumerate() {
        // test if first 7 bit are set, to check for overflow
        if (tmp & 0xfe_00_00_00) > 0 {
            return Err(Error::Overflow);
        }

        // append bytes of b to tmp
        tmp = (tmp << 7) | u32::from(b & 0x7f);

        if 0x80 & b == 0 {
            // continuation bit not set, number if complete
            res.push(tmp);
            tmp = 0;
        } else {
            // check for incomplete bytes
            if i + 1 == bytes.len() {
                // the next index would be past the end,
                // i.e. there are no more bytes.
                return Err(Error::IncompleteNumber);
            }
        }
    }

    Ok(res)
}
