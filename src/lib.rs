pub fn count(lines: &[&str]) -> u32 {
    //unimplemented!("\nDetermine the count of rectangles in the ASCII diagram represented by the following lines:\n{lines:#?}\n.");

    // grid  is a matrix;
    // To create and work with a grid in Rust, you generally use a vector of vectors (Vec<Vec<T>>),
    // where T is the type of the elements in the grid.
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // lines.iter() creates an iterator over the lines vector.
    // map(|line| line.chars().collect()) maps each line to a vector of characters.
    // collect() collects the mapped vectors into a vector of vectors.

    let mut count = 0;
    for x1 in 0..grid.len() {
        for y1 in 0..grid[0].len() {
            for x2 in (x1 + 1)..grid.len() {
                for y2 in (y1 + 1)..grid[0].len() {
                    if is_rectangle(&grid, x1, y1, x2, y2) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}


fn is_rectangle(grid: &[Vec<char>], x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    // Check if the corners are '+'.
    if [grid[x1][y1], grid[x1][y2], grid[x2][y1], grid[x2][y2]].iter().any(|&c| c != '+') {
        return false;
    }

    // Check the top and bottom sides.
    for j in (y1 + 1)..y2 {
        if [grid[x1][j], grid[x2][j]].iter().any(|&c| c != '-' && c != '+') {
            return false;
        }
    }

    // Check the left and right sides.
    for i in (x1 + 1)..x2 {
        if [grid[i][y1], grid[i][y2]].iter().any(|&c| c != '|' && c != '+') {
            return false;
        }
    }

    true
}