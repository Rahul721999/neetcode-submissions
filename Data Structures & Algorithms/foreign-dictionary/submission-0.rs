impl Solution {
    pub fn foreign_dictionary(words: Vec<String>) -> String {
        let mut res = "".to_string();
        let n = words.len();

        use std::collections::{HashMap, HashSet, VecDeque};
        let mut edges: HashMap<char, HashSet<char>> = HashMap::new();
        let mut indegree: HashMap<char, i32> = HashMap::new();

        for word in &words {
            for ch in word.chars() {
                indegree.entry(ch).insert_entry(0);
            }
        }

        for i in 0..n - 1 {
            let word1: Vec<char> = words[i].clone().chars().collect();
            let word2: Vec<char> = words[i + 1].clone().chars().collect();

            if word1.is_empty() || word2.is_empty() {
                continue;
            }
            let mut p = 0_usize;
            let min_len = std::cmp::min(word1.len(), word2.len());
            while p < min_len && word1[p] == word2[p] {
                p += 1;
                continue;
            }

            // if all the char maches but w1.len() > w2.len()
            // means it's invalid dictionary
            if p == min_len && word1.len() > word2.len() {
                return "".to_string();
            }

            if p < min_len {
                let (src, dest) = (word1[p], word2[p]);
                let inserted = if let Some(node) = edges.get_mut(&src) {
                    node.insert(dest)
                } else {
                    let mut new_set = HashSet::new();
                    let inserted = new_set.insert(dest);
                    edges.insert(src, new_set);
                    inserted
                };

                if inserted {
                    *indegree.get_mut(&dest).unwrap() += 1;
                }
            }
        }

        let mut queue: VecDeque<char> = VecDeque::new();
        // push nodes to queue having indegree 0
        for (node, degree) in indegree.iter() {
            if *degree == 0 {
                queue.push_front(*node);
            }
        }

        let mut processed = indegree.len();

        // listed the queue now
        while let Some(node) = queue.pop_back() {
            res.push(node);
            processed -= 1;

            // for all the neighbers reduce the count
            if let Some(dests) = edges.get(&node) {
                for dest in dests {
                    if let Some(degree) = indegree.get_mut(&dest) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_front(*dest);
                        }
                    } else {
                        panic!("Should not happen");
                    }
                }
            }
        }

        if processed != 0 {
            return "".to_string();
        }
        res
    }
}
