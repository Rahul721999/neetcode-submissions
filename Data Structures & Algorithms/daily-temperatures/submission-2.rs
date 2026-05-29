impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let len = t.len();
        let mut stack: Vec<usize> = Vec::new();

        let mut res: Vec<i32> = vec![0; len];

        for i in (0..len).rev(){
            if !stack.is_empty() {
                let curr = t[i];
                while !stack.is_empty() && curr >= t[*stack.last().unwrap()]{
                    stack.pop();
                }
                if let Some(val) = stack.last(){
                    res[i] = (val - i) as i32;
                }
            }
            stack.push(i);
        }
        res
    }
}
