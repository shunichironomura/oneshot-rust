pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    'search: {
        for (r, row) in grid.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                if value == target {
                    break 'search Some((r, c));
                }
            }
        }
        None
    }
}
