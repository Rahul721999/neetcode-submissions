impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0;
        let m = n.unsigned_abs();
        for i in 0..m{
            res = res * x;
        }
        if n < 0{
            return 1.0 / res
        }else{
            res
        }
    }
}
