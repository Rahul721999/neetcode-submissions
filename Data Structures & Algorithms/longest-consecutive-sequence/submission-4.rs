impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 0 {return 0}

        let mut nums = nums.clone();
        nums.sort();

        let mut i = 0;
        let mut max = 1;
        let mut count = 1;
        while i < len - 1{
            if nums[i] == nums[i + 1]{ // handling duplicates
                i += 1; 
                continue;
            }
            if nums[i] + 1 == nums[i + 1]{ // check if the next ele is present
                count += 1;
            }else{
                count = 1;
            };
            i += 1;
            max = max.max(count);
        }
        max
    }
}
