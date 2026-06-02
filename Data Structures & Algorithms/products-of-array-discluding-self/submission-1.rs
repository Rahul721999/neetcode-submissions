impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut prefix = vec![1; len];

        for i in 1..len{
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        let mut suffix = vec![1; len];

        for i in (0..len - 1).rev(){
            suffix[i] = suffix[i + 1] * nums[i+1];
        }

        let mut res: Vec<i32> = Vec::new();
        for i in 0..len{
            res.push(prefix[i] * suffix[i]);
        }
        res
    }
}
