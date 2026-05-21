impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut prefix = vec![0; len];

        // initializing..
        prefix[0] = 1;

        // prepare prefix array
        for i in 1..len {
            prefix[i] = nums[i-1] * prefix[i-1];
        }


        let mut suffix = vec![0;len];

     // initializing..
        suffix[len-1] = 1;
   
        // preparing suffix array
        for i in (0..len-1).rev(){
            suffix[i] = suffix[i+1] * nums[i+1];
        }

        // prepare the result vector now
        let mut res = vec![0;len];
        for i in 0..len {
            res[i] = prefix[i] * suffix[i];
        }

        res
    }
}