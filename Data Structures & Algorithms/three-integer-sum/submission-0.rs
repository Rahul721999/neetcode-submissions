impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();

        if len < 3 {return result}
        nums.sort();

        for (i, ele) in nums.iter().enumerate() {
            if i > 0 && nums[i] == nums[i-1]{
                    continue
                }
            let target = - *ele;

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                
                let sum = nums[left] + nums[right];
                if sum < target {
                    left += 1;
                }else if sum > target{
                    right -= 1;
                }else {
                    let res = vec![*ele, nums[left], nums[right]];
                    result.push(res);
                    right -= 1;
                    left += 1;

                    // skip duplicates
                    while left < right && nums[left] == nums[left- 1]{ left += 1;}
                    while left < right && nums[right] == nums[right + 1]{ right -= 1;}
                }
            }
        }

        result
    }
}
