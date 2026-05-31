impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        for n in nums{
            heap.push(n);
        };

        for _ in 0..k-1{
            heap.pop();
        };
        if let Some(val) = heap.pop(){
            return val;
        }
        0
    }
}
