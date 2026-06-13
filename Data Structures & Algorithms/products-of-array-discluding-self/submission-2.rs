impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = vec![1;len];
        let mut right = vec![1;len];

        for i in 1..len{
            left[i] = left[i-1] * nums[i -1];
        }

        for i in (0..len - 1).rev(){
            right[i] = right[i+1] * nums[i + 1];
        }
        
        let mut res = vec![0;len];
        for i in 0..len{
            res[i] = left[i] * right[i];
        }
        res
    }
}
