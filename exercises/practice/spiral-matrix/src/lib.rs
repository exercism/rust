pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut size: usize = size as usize;
    let mut start_count = 1;
    let mut start_pos = (0, 0);

    let mut res: Vec<Vec<u32>> = vec![vec![0; size]; size];

    loop {

        // Empty square, return
        if size == 0 {
            return res;
        }

        // Upper segment
        for k in 0..size {
            res[start_pos.0][start_pos.1 + k] = start_count;
            start_count+=1;
        }

        // Single cell square. 
        if size == 1 {
            break;
        }

        // Right segment
        for k in 1..size {
            res[start_pos.0 + k][start_pos.1 + size-1] = start_count;
            start_count+=1;
        }

        // Lower segment
        for k in (0..=(size-2)).rev() {
            res[start_pos.0 + size-1][start_pos.1 + k] = start_count;
            start_count+=1;
        }

        // Left segment
        for k in (1..=(size-2)).rev() {
            res[start_pos.0 + k][start_pos.1] = start_count;
            start_count+=1;
        }

        if size <= 2 {
            break;
        }
        size -= 2;
        start_pos = (start_pos.0+1, start_pos.1 + 1);
    }


    res

}
