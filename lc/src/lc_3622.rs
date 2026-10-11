pub fn lc_3622_is_divisible(n: u32) -> bool {
    let ns = n.to_string();
    let mut s: u32 = 0;
    let mut p: u32 = 1;
    for c in ns.chars() {
        let ci = c.to_digit(10).unwrap_or(0);
        s += ci;
        p *= ci;
    }
    n % (s + p) == 0
}
