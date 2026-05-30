impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<char> = HashSet::new();

        let s: Vec<char> = s.chars().collect();

        let mut i = 0;
        let mut j = 0;

        let mut max = 0;
        while j< s.len(){
            if !set.contains(&s[j]){
                set.insert(s[j]);
                j += 1;
                max = max.max(set.len());
            }else{
                while set.contains(&s[j]){
                    set.remove(&s[i]);
                    i += 1;
                }
            }
        }
        max as i32
    }
}
