impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        let mut i: i32 = 0;

        while i <= r{
            if nums[i as usize] == 0{
                let temp = nums[l as usize];
                nums[l as usize] = nums[i as usize];
                nums[i as usize] = temp;
                l += 1;
            }
            else if nums[i as usize] == 2{
                let temp = nums[r as usize];
                nums[r as usize] = nums[i as usize];
                nums[i as usize] = temp;
                r -= 1;
                i -= 1;
            }
            i += 1;
        }
    }
}
