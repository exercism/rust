pub fn egg_count(display_value: u32) -> usize {
    let mut display_value = display_value;
    let mut count: u32 = 0;

    while display_value != 0 {
        count += display_value & 1;
        display_value >>= 1;
    }
    count.try_into().unwrap()
}
