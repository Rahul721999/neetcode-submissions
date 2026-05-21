impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, n) in nums.iter().enumerate(){
            map.insert(*n, i as i32);
        }

        for (i,n) in nums.iter().enumerate() {
            let i = i as i32;
            let rem = target - n;
            if let Some(v) =  map.get(&rem) {
                if i == *v {continue}
                res.push(i);
                res.push(*v);
                return res
            }
        }
        res
    }
}
