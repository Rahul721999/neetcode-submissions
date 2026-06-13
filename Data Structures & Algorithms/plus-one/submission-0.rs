impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut n: i64 = 0;
        for &d in digits.iter(){
            n = n * 10 + d as i64;
        }

        // plus one 
        n += 1;

        // convert to array again
        let mut res: Vec<i32> = Vec::new();
        while n > 0{
            let e = (n % 10) as i32;
            n = n/10;
            res.push(e);
        }
        res.reverse();
        res
    }
}
