impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right = nums.len() as i32 - 1;

        while left < right{
            let mid = left + (right - left)/2;
            let mid_value = nums[mid as usize];

            if mid_value > nums[right as usize] {
                left = mid + 1;
            }else{
                right = mid // including the mid value
            }
        };
        let min = nums[left as usize];
        (left, right) = if target > min && target > nums[nums.len() - 1]{
            (0, left)
        }else{
            (left, nums.len() as i32 - 1)
        };

        while left <= right {
            let mid = left + (right - left)/2;
            let mid_value = nums[mid as usize];
            if mid_value > target{
                right = mid - 1;
            }else if mid_value < target{
                left = mid + 1;
            }else{
                return mid
            }
        }
        - 1
    }
}
