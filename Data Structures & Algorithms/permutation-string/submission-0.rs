impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let window_len = s1.len();
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        let mut s1_char_count: Vec<i32> = vec![0; 26];
        s1.iter().for_each(|&ch| 
            {s1_char_count[(ch as u8 - b'a') as usize] += 1;}
        );

        let mut left = 0;
        let mut right = 0;

        let mut s2_char_count: Vec<i32> = vec![0; 26];

        while right < s2.len(){
            let index = s2[right] as u8 - b'a';
            s2_char_count[index as usize] += 1;

            if (right-left) + 1 > window_len{
                let index = s2[left] as u8 - b'a';
                s2_char_count[index as usize] -= 1;
                left += 1;
            }

            if right - left + 1 == window_len {
                if s1_char_count == s2_char_count{
                    return true
                }
            }
            right += 1;
        }
        false
    }
}
