impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums.iter(){
            *map.entry(*n).or_insert(0) += 1;
        }

        // collect everything inside vector
        let mut items: Vec<(i32,i32)> = map.into_iter().map(|(k,v)| (k,v)).collect();

        // short by value
        items.sort_by(|a,b| b.1.cmp(&a.1));

        items.into_iter().take(k as usize).map(|(num, _)| num).collect()
    }
}
