pub fn uniform_array2(nums1: Vec<i32>) -> bool {
    let mut can_even: usize = 0;
    let mut can_odd: usize = 0;

    let mut nums2 = nums1.clone();
    nums2.sort();

    let mut smollest_odd = i32::MAX;

    for n in nums2 {
        if n < smollest_odd && n % 2 != 0 {
            smollest_odd = n;
        }

        if n % 2 == 0 {
            can_even += 1;
            if smollest_odd != -1 && n - smollest_odd >= 1 {
                can_odd += 1;
            }
        } else {
            can_odd += 1;
            if smollest_odd != -1 && n - smollest_odd > 1 {
                can_even += 1;
            }
        }
    }

    nums1.len() == can_even || nums1.len() == can_odd
}
