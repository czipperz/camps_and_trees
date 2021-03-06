use board::*;
use tile::Tile::*;

/// Fill rows and columns with no remaining [`Camp`]s with [`Grass`].
///
/// Return whether any values were changed.
///
/// # Examples
///
/// ```
/// # use camps_and_trees::{Board, fill_zeros};
/// let mut board = Board::new_parse(vec![1, 2, 1], vec![1, 1, 2], "   \n CC\n   ").unwrap();
/// assert!(fill_zeros(&mut board));
/// assert_eq!(board.debug(), " - \n-CC\n - ");
/// ```
///
/// [`Camp`]: enum.Tile.html#variant.Camp
/// [`Grass`]: enum.Tile.html#variant.Grass
pub fn fill_zeros(board: &mut Board) -> bool {
    let mut changed = false;
    for row in 0..board.rows.len() {
        if board.count_in_row(row, Camp) == board.rows[row] {
            for column in 0..board.columns.len() {
                if board.grid[(row, column)] == Unassigned {
                    board.grid[(row, column)] = Grass;
                    changed = true;
                }
            }
        }
    }
    for column in 0..board.columns.len() {
        if board.count_in_column(column, Camp) == board.columns[column] {
            for row in 0..board.rows.len() {
                if board.grid[(row, column)] == Unassigned {
                    board.grid[(row, column)] = Grass;
                    changed = true;
                }
            }
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_zeros_0_camps() {
        let mut board = Board::new_parse(vec![0, 1, 1], vec![1, 2, 0], "   \n   \n   ").unwrap();
        assert!(fill_zeros(&mut board));
        assert_eq!(board.debug(), "---\n  -\n  -");
    }

    #[test]
    fn fill_zeros_column_with_a_camp() {
        let mut board = Board::new_parse(vec![1, 2, 1], vec![1, 1, 2], "   \n CC\n   ").unwrap();
        assert!(fill_zeros(&mut board));
        assert_eq!(board.debug(), " - \n-CC\n - ");
    }

    #[test]
    fn fill_zeros_row_with_multiple_camps() {
        let mut board = Board::new_parse(vec![2, 2, 1], vec![2, 2, 2], "C C\n   \n   ").unwrap();
        assert!(fill_zeros(&mut board));
        assert_eq!(board.debug(), "C-C\n   \n   ");
    }
}
