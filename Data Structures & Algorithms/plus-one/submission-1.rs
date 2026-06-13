impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        // iterate in reverse and process each digit
        for i in (0..digits.len()).rev(){

            // if the digit < 9 then it's fine to add 1
            if digits[i] < 9{
                digits[i] += 1;
                return digits;
            }
            // else adding 1 to 9 will be 10; so keep 0, and add 1 to it's prev digit
            digits[i] = 0;
        }

        // if code reaches at this point means. all the value were 9; e.g. (9,9,9)
        // then we need to add 1 at the front: (0,0,0) => (1,0,0,0);
        digits.insert(0,1);
        digits
    }
}
