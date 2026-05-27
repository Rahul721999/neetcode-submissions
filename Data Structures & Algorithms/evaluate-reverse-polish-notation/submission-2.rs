impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for s in tokens{
            if !matches!(s.as_str(), "+" | "-" | "*" | "/") {
                stack.push(s.parse::<>().unwrap());
            }else{
                if stack.len() >= 2{
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();
                    let res = match s.as_str(){
                        "+" => v1 + v2,
                        "-" => v1 - v2,
                        "*" => v1 * v2,
                        "/" => v1/v2,
                        _ => panic!("unreachable"),
                    };
                    stack.push(res);
                }
            }
        }
        
        if stack.is_empty() {
            return 0
        };
        stack.pop().unwrap()
    }
}
