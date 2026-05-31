impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let height = matrix.len();
        let width = matrix[0].len();

        for row in 0..height{
            for col in row..width{ // be carefull here
                // skipping diagonals, which will anyway wont swap.
                if col != row{
                    // swapping / transpossing..
                    let temp = matrix[row][col];
                    matrix[row][col] = matrix[col][row];
                    matrix[col][row] = temp;
                }
            }
        }

        // reverse each row now...
        for row in 0..height{
            matrix[row].reverse();
        }
    }
}
