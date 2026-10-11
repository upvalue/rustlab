use std::cmp;

pub fn lc_2091_remove_min_and_max_from_array(nums: Vec<i32>) -> i32 {
    // find min number, max number idx -- only need min_num max_num as intermediary
    let min_idx = nums.iter().enumerate().min_by_key(|&(_, x)| x).unwrap().0;
    let max_idx = nums.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;

    let front_idx = cmp::min(min_idx, max_idx) as i32;
    let back_idx = cmp::max(min_idx, max_idx) as i32;
    let nums_len = nums.len() as i32;

    (back_idx + 1)
        .min(front_idx + 1 + nums_len - back_idx)
        .min(nums_len - front_idx)
}

//let (min_idx, max_idx) = nums.iter().fold((0, 0), |acc, &n| (n, n));
// println!("min_idx: {}", min_idx);
/*let mut min_idx = 0;
let mut max_idx = 0;
let mut min_num: i32 = 2147483647;
let mut max_num: i32 = -1;
for (idx, n) in nums.iter().enumerate() {
    if n < &min_num {
        min_idx = idx;
        min_num = *n;
    }
    if n > &max_num {
        max_idx = idx;
        max_num = *n;
    }
}

if max_idx == min_idx {
    (cmp::min(min_idx, nums.len() - min_idx) + 1) as i32
} else {
    let front_idx = cmp::min(min_idx, max_idx);
    let back_idx = cmp::max(min_idx, max_idx);

    // println!("{:?}", nums);
    // println!("front_idx={} back_idx={}", front_idx, back_idx);

    let res = cmp::min(
        (front_idx + (back_idx - front_idx)) + 1,
        cmp::min(
            (front_idx + (nums.len() - back_idx)) + 1,
            nums.len() - front_idx,
        ),
    ) as i32;

    // println!("res={}", res);

    res
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_2091() {
        // case 1
        assert_eq!(
            lc_2091_remove_min_and_max_from_array(vec![2, 10, 7, 5, 4, 1, 8, 6]),
            5
        );
        // case 2
        assert_eq!(
            lc_2091_remove_min_and_max_from_array(vec![0, -4, 19, 1, 8, -2, -3, 5]),
            3
        );
        // case 3
        assert_eq!(lc_2091_remove_min_and_max_from_array(vec![101]), 1);
    }
}
