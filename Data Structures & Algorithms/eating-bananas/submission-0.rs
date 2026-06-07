
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_pile = 0;
        for p in piles.iter(){
            max_pile = max_pile.max(*p);
        }

        let mut left = 1;
        let mut right = max_pile;

        while left < right{
            let mid = left + (right - left) / 2;
            let mut hour = 0; // total hour required to finish all the piles

            for p in piles.iter(){
                hour += (p + mid - 1) / mid;
            }

            if hour > h { // we are too slow
                left = mid + 1;
            }else{
                right = mid;
            }
        }
        left
    }
}
