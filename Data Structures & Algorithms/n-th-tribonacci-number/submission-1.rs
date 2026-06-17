impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;

        match n{
            0 => return 0,
            1 => return 1,
            2 => return 1,
            _ => {
                let mut res = 0;
                for i in 3..=n{
                    let d = a + b + c;
                    a = b;
                    b = c;
                    c = d;
                    res = d;
                }
                return res
            }
        }
        a
    }
}

// 0 = 0
// 1 = 1
// 2 = 1
// 3 = 2
// 4 = 4
// 5 = 7
// 6 = 14