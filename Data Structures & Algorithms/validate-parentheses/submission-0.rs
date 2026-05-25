impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;

        let mut stack: VecDeque<char> = VecDeque::new();

        let s: Vec<char> = s.chars().collect();

        let opening = vec!['(', '{', '['];
        for char in s.iter() {
            if opening.contains(char) {
                stack.push_back(*char);
            } else {
                if let Some(poped) = stack.pop_back() {
                    match char {
                        ')' => {
                            if poped != '(' {
                                return false;
                            }
                        }
                        '}' => {
                            if poped != '{' {
                                return false;
                            }
                        }
                        ']' => {
                            if poped != '[' {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
        }
        if stack.len() > 0{
            return false
        }
        true
    }
}
