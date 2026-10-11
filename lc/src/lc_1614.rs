pub fn lc_1614_max_depth(s: String) -> i32 {
    let mut pc: i32 = 0;
    let mut max_pc: i32 = 0;
    for c in s.as_bytes() {
        if *c == b'(' {
            pc += 1;
            max_pc = if pc > max_pc { pc } else { max_pc }
        } else if *c == b')' {
            pc -= 1;
        }
    }
    max_pc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_1614() {
        // case 1
        assert_eq!(lc_1614_max_depth("(1+(2*3)+((8)/4))+1".into()), 3);
    }
}
