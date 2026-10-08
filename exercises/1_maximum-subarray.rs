impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sol = nums[0];
        let mut partial_sum = 0;
        for &num in &nums {
            partial_sum += num;
            sol = sol.max(partial_sum);
            partial_sum = partial_sum.max(0);
        }

        sol
    }
}
