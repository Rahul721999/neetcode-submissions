impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 0 {return 0}

        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();

        for &ele in &nums{
            set.insert(ele);
        }

        let mut max = 0;
        for &ele in &nums {
            if !set.contains(&(ele - 1)){
                let mut count = 1;
                let mut e = ele;
                while set.contains(&(e + 1)){
                    count += 1;
                    e += 1;
                };
                max = max.max(count);
            }
        }
        max
    }
}
