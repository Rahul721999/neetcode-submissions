impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0{return 0}
        if n == 1{return 1}
        if n == 2{return 1}
        let mut arr = vec![0; n as usize + 1];
        arr[0] = 0;
        arr[1] = 1;
        arr[2] = 1;

        for i in 3..=n as usize{
            arr[i] = arr[i-1] + arr[i-2] + arr[i - 3];
        }
        arr[n as usize]
    }
}

// 0 = 0
// 1 = 1
// 2 = 1
// 3 = 2
// 4 = 4
// 5 = 7
// 6 = 14