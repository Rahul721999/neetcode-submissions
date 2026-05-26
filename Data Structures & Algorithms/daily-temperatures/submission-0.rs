impl Solution {
    pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
        let len = temp.len();

        let mut stack: Vec<usize> = Vec::new();

        let mut res = vec![0;len];
        
        for i in (0..len).rev(){
            let curr = temp[i];

            while stack.len() > 0 && curr >= temp[*stack.last().unwrap()]{
                stack.pop();
            }

            if !stack.is_empty() {
                res[i] = (stack.last().unwrap() - i) as i32;
            }

            stack.push(i);
        }
        res
    }
}
