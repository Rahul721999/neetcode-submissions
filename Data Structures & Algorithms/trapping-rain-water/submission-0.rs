impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut l2r: Vec<i32> = vec![0;len];
        let mut r2l: Vec<i32> = vec![0;len];

        let mut max = 0;
        for (i, ele) in height.iter().enumerate(){
            max = max.max(*ele);
            l2r[i] = max;
        }

        max = 0;
        for i in (0..len).rev(){
            max = max.max(height[i]);
            r2l[i] = max;
        }

        let mut res = 0;
        for i in 0..len{
            let min = l2r[i].min(r2l[i]);
            res += min - height[i];
        }

        res
    }
}
// [0,2,2,3,3,3,3,3,3,3] // l2r
// [3,3,3,3,3,3,3,3,2,1] // r2l

// [0,2,0,3,1,0,1,3,2,1] // original

// [0,0,2,0,2,3,2,0,0,0]
