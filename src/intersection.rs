use board::*;
use grid::*;
use tile::Tile::*;

/// Process a single row of the `Grid`.
///
/// # Steps
///
/// 1. If `count == 0` then push `grid` onto `possibilities` and
///    return.
/// 2. If `column == grid.num_columns()` then simply return.  This is
///    because there weren't enough [`Camp`]s placed for this
///    possibility to be valid.
/// 3. Otherwise, try putting a [`Camp`] at `(row, column)`.  If that
///    succeeds, recurse into `(row, column + 1)` with that grid.
/// 4. Recurse into `(row, column + 1)` without placing a [`Camp`] at
///    `(row, column)`.
fn process_row(possibilities: &mut Vec<Grid>, grid: Grid, count: usize, row: usize, column: usize) {
    if count == 0 {
        possibilities.push(grid);
        return;
    } else if column == grid.num_columns() {
        return;
    } else if grid[(row, column)] == Unassigned {
        // try assigning here
        let mut b = grid.clone();
        if b.set_camp(row, column).is_ok() {
            process_row(possibilities, b, count - 1, row, column + 1);
        }
    }
    // don't assign here
    process_row(possibilities, grid, count, row, column + 1)
}

/// See documentation for `process_row`.
fn process_column(
    possibilities: &mut Vec<Grid>,
    grid: Grid,
    count: usize,
    row: usize,
    column: usize,
) {
    if count == 0 {
        possibilities.push(grid);
        return;
    } else if row == grid.num_rows() {
        return;
    } else if grid[(row, column)] == Unassigned {
        // try assigning here
        let mut b = grid.clone();
        if b.set_camp(row, column).is_ok() {
            process_column(possibilities, b, count - 1, row + 1, column);
        }
    }
    // don't assign here
    process_column(possibilities, grid, count, row + 1, column)
}

/// Find the intersection of all possibilities.
///
/// If a [`Tile`] has the same value throughout each possibility, then
/// that [`Tile`] is yielded the same way in the resulting [`Grid`].
/// If it varies, then it is [`Unassigned`].
///
/// [`Tile`]: enum.Tile.html
/// [`Grid`]: struct.Grid.html
/// [`Unassigned`]: enum.Tile.html#variant.Unassigned
fn intersection(possibilities: Vec<Grid>) -> Grid {
    let mut possibilities = possibilities.into_iter();
    let mut grid = possibilities.next().unwrap();
    for ngrid in possibilities {
        for row in 0..grid.num_rows() {
            for column in 0..grid.num_columns() {
                if grid[(row, column)] != ngrid[(row, column)] {
                    grid[(row, column)] = Unassigned;
                }
            }
        }
    }
    grid
}

/// Loop through every possibility for each column and row and process
/// their intersections.
pub fn process_intersections(board: &mut Board) -> bool {
    let mut changed = false;
    for row in 0..board.rows.len() {
        let mut possibilities = Vec::new();
        let count = board.rows[row] - board.count_in_row(row, Camp);
        process_row(&mut possibilities, board.grid.clone(), count, row, 0);
        let new_grid = intersection(possibilities);
        changed = changed || board.grid != new_grid;
        board.grid = new_grid;
    }
    for column in 0..board.columns.len() {
        let mut possibilities = Vec::new();
        let count = board.columns[column] - board.count_in_column(column, Camp);
        process_column(&mut possibilities, board.grid.clone(), count, 0, column);
        let new_grid = intersection(possibilities);
        changed = changed || board.grid != new_grid;
        board.grid = new_grid;
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_one_possibility_is_the_possibility() {
        let grid = Grid::blank(3, 3);
        assert_eq!(intersection(vec![grid.clone()]), grid);
    }

    #[test]
    fn intersection_two_possibilities() {
        let grid1 = Grid::parse(" T \n C-\n-  ").unwrap();
        let grid2 = Grid::parse("CT \n C-\n   ").unwrap();
        assert_eq!(
            intersection(vec![grid1, grid2]),
            Grid::parse(" T \n C-\n   ").unwrap()
        );
    }

    #[test]
    fn process_intersections_row_deduce_grass_next_row() {
        let mut board = Board::new_parse(
            vec![1, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 0],
            " - --\nT T  \n-    \n     \n     ",
        ).unwrap();
        assert!(process_intersections(&mut board));
        assert_eq!(board.debug(), " - --\nT-T  \n-    \n     \n     ");
    }

    #[test]
    fn process_intersections_column_deduce_grass_next_column() {
        let mut board = Board::new_parse(
            vec![1, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            " T   \n-    \n T   \n-    \n-    ",
        ).unwrap();
        assert!(process_intersections(&mut board));
        assert_eq!(board.debug(), " T   \n--   \n T   \n-    \n-    ");
    }
}
