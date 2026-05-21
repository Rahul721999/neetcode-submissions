impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 0 {
            return 0
        }

        let mut nums = nums.clone();
        nums.sort();


        let mut curr = 0;
        let mut max_so_far = 1;
        let mut count = 1;

        while curr < len - 1 {
            if nums[curr] == nums[curr + 1] {
                curr += 1;
                continue;
            }
            if nums[curr] + 1 == nums[curr + 1] {
                count += 1;
            } else {
                count = 1;
            }

            max_so_far = count.max(max_so_far);
            curr += 1;
        }

        max_so_far
    }
}
