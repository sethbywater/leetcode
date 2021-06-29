pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = vec![vec![1; n]; n];
    for pair in mines {
        grid[pair[0] as usize][pair[1] as usize] = 0
    }
  
    let mut max = 0;

    for i in 0..n {
        for j in 0..n {
            let mut k: usize = grid[i][j] as usize;
            while k <= i            // Make sure we don't fall off the grid
                && k + i < n
                && k <= j
                && k + j < n
                && grid[i-k][j] > 0 // And that we don't hit a mine
                && grid[i+k][j] > 0
                && grid[i][j-k] > 0
                && grid[i][j+k] > 0
            {
                k += 1
            }

            max = std::cmp::max(max, k as i32)
        }
    }

    max
}

#[test]
fn example_1() {
    assert_eq!(order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2)
}

#[test]
fn example_2() {
    assert_eq!(order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0)
}

#[test]
// First test case I failed, because I did not check edges
fn edges() {
    assert_eq!(
        order_of_largest_plus_sign(3, vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
            vec![2, 0],
            vec![2, 1],
            vec![2, 2],
        ]),
        1
    )
}
