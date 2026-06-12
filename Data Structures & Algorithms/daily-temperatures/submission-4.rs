impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let len = t.len();
        let mut res: Vec<i32> = vec![0; len];

        for i in (0..len).rev(){
            let curr_temp = t[i];
            while let Some(j) = stack.last() && curr_temp >= t[*stack.last().unwrap()]{
                let p = stack.pop();
                println!("{p:?}");
            }
            if !stack.is_empty(){
                res[i] = (stack.last().unwrap() - i) as i32;
            }else{
                res[i] = 0;
            }
            stack.push(i);
        }
        res
    }
}
