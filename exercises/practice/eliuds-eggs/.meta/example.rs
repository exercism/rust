pub fn egg_count(display_value: u32) -> usize {
    (0..32).filter(|i| display_value & (1 << i) != 0).count()
}
