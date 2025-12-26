#[must_use]
#[inline]
pub const fn neighbors(row: usize, col: usize) -> [(usize, usize); 8] {
    [
        (row, col.wrapping_sub(1)),
        (row, col.wrapping_add(1)),
        (row.wrapping_sub(1), col),
        (row.wrapping_add(1), col),
        (row.wrapping_sub(1), col.wrapping_sub(1)),
        (row.wrapping_sub(1), col.wrapping_add(1)),
        (row.wrapping_add(1), col.wrapping_sub(1)),
        (row.wrapping_add(1), col.wrapping_add(1)),
    ]
}

#[must_use]
#[inline]
pub fn get_grid(inp: &str) -> Vec<Vec<u8>> {
    inp.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}
