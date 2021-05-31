//! # 867. Transpose matrix
//!
//! Given a 2D integer matrix, return the transpose of the matrix

/// Transposes a 2D matrix
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	let mut res: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];
	for row in 0..matrix.len() {
		for col in 0..matrix[0].len() {
			res[col][row] = matrix[row][col]
		}
	}
	res
}