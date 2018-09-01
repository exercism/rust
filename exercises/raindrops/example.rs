pub fn raindrops(n: u32) -> String {
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;
    let mut drops = String::new();
    if is_pling(n) {
        drops.push_str("Pling");
    }
    if is_plang(n) {
        drops.push_str("Plang");
    }
    if is_plong(n) {
        drops.push_str("Plong");
    }
    if drops.is_empty() {
        let s = format!("{}", n);
        drops.push_str(&s);
    }
    drops
}
