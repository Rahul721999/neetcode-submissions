impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();

        // part 1. find out the smallest ele first
        let mut left = 0 as i32;
        let mut right = len as i32 - 1;
        while right > left {
            let mid = left + (right - left) / 2;
            let mid_value = nums[mid as usize];
            if mid_value < nums[right as usize]{
                right = mid;
            }else{
                left = mid + 1;
            }
        }

        let smallest_value = nums[left as usize];

        // part 2: check the target now
        (left, right) = if target >= smallest_value && target <= nums[len -1]{
            (left, len as i32)
        }else{
            (0 as i32,left - 1)
        };

        while right >= left {
            let mid = left + (right - left) / 2;
            let mid_value = nums[mid as usize];
            if mid_value == target{
                return mid;
            }
            if target > mid_value {
                left = mid + 1;
            }else{
                right = mid - 1;
            }
        }
        -1
    }
}
