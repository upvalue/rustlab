pub fn lc_2778_sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut totes = 0;
    for i in 0..nums.len() {
        if (nums.len() % (i + 1)) == 0 {
            totes += nums[i] * nums[i];
        }
    }
    totes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_2778() {
        assert_eq!(lc_2778_sum_of_squares(vec![1, 2, 3, 4]), 21);
    }
}
