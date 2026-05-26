impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l = 0_usize;
        let mut r = heights.len() - 1;

        let mut max = i32::MIN;
        while l < r {
            let width = (r - l) as i32;
            let height = heights[r].min(heights[l]);
            let area = width * height;

            max = area.max(max);

            if heights[r] < heights[l] {
                r -= 1;
            }else{
                l += 1;
            }
        }
        max
    }
}
