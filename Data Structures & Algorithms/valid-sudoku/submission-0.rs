impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        
        let height = board.len();
        let width = board[0].len();

        // check in the row
        for row in 0..height {
            let mut row_set: HashSet<char> = HashSet::new();
            for col in 0..width{
                let val = board[row][col];
                if val != '.'{
                    if let false = row_set.insert(val){
                        return false;
                    }
                }
            }
        }

        // check in col
        for col in 0..width{
            let mut col_set: HashSet<char> = HashSet::new();
            for row in 0..height{
                let val = board[row][col];
                if val != '.'{
                    if let false = col_set.insert(val){
                        return false;
                    }
                }
            }
        }

        // check in the grid
        for box_row in (0..9).step_by(3){
            for box_col in (0..9).step_by(3){
                let mut grid_set: HashSet<char> = HashSet::new();
                for row in box_row..box_row + 3{
                    for col in box_col..box_col + 3 {
                        let val = board[row][col];
                        if val != '.'{
                            if let false = grid_set.insert(val){
                                return false;
                            }
                        }
                    }
                }
                
            }
        }
        true
    }
}
