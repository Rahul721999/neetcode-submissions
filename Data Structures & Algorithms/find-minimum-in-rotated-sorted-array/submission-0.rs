impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let mut left = 0 as i32;
        let mut right = len as i32 - 1;

        while left < right{
            let pivot = left + (right - left)/2;
            let p_val = nums[pivot as usize];

            // if mid is larger the right most ele, means the rotated indx is on right
            if p_val > nums[right as usize]{
                left = pivot + 1;
            }else{ // else search on the left.
                right = pivot;
            }
        }
        nums[left as usize]
    }
}
