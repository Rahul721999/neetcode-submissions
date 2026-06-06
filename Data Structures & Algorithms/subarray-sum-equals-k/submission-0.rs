impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut count = 0 ;
        let mut prefix: Vec<i32> = vec![0; len];

        // initialize prefix 
        prefix[0] = nums[0_usize];
        for i in 1..len{
            prefix[i] = prefix[i-1] + nums[i];
        }

        use std::collections::HashMap;
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for j in 0..len{
            if prefix[j] == k {
                count += 1;
            };
            let val = prefix[j] - k;
            if let Some(freq) = freq_map.get(&val){
                count += freq;
            }
            *freq_map.entry(prefix[j]).or_insert(0) += 1;
        }

        count
    }
}