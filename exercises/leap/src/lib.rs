pub fn is_leap_year(year: u64) -> bool {
    let f0 = |n| year % n == 0;

    (!f0(100) || f0(400)) && f0(4)
//    return false;
}
