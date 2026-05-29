impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let height = matrix.len();
        let width = matrix[0].len();

        let mut left = 0 as i32;
        let mut right = height as i32 - 1;
        let mut row = -1;

        while left <= right {
            let pivot = left + (right - left) / 2;
            let first = matrix[pivot as usize][0];
            let last = matrix[pivot as usize][width - 1];
            if first == target {
                return true;
            }
            if target > last {
                left = pivot + 1;
            } else if target < first {
                right = pivot - 1;
            } else {
                row = pivot;
                break;
            };
        }

        if row == -1 {
            return false;
        }

        let mut left = 1 as i32; // because we've already searched 0th col
        let mut right = width as i32 - 1;

        while left <= right {
            let pivot = left + (right - left) / 2;
            let mid_val = matrix[row as usize][pivot as usize];
            if mid_val == target {
                return true;
            }
            if target > mid_val {
                left = pivot + 1;
            } else {
                right = pivot - 1;
            }
        }

        false
    }
}
