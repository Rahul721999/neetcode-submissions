impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();

        let mut left  = 0;
        let mut right = len - 1;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum > target {
                right -= 1;
            }
            else if sum < target{
                left += 1;
            }
            else {
                return vec![left as i32 + 1, right as i32 + 1];
            }
        }
        vec![]
    }
}
