use board::*;
use tile::Tile::*;

/// Fill rows and columns with `Camp`s where there are `Unassigned`
/// slots.
///
/// Return whether any values were changed.
///
/// # Examples
///
/// Here there are exactly 2 `Unassigned` slots and 2 `Camp`s left to
/// place so we place them:
///
/// ```
/// # use camps_and_trees::{Board, fill_camps};
/// let mut board = Board::new_parse(vec![2, 0, 2], vec![2, 0, 2], " T \nT-T\n T ").unwrap();
/// assert!(fill_camps(&mut board));
/// assert_eq!(board.debug(), "CTC\nT-T\nCTC");
/// ```
pub fn fill_camps(board: &mut Board) -> bool {
    let mut changed = false;
    for row in 0..board.rows.len() {
        if board.rows[row] == board.count_in_row(row, Unassigned) + board.count_in_row(row, Camp) {
            for column in 0..board.columns.len() {
                if board.grid[(row, column)] == Unassigned {
                    board.grid[(row, column)] = Camp;
                    changed = true;
                }
            }
        }
    }
    for column in 0..board.columns.len() {
        if board.columns[column]
            == board.count_in_column(column, Unassigned) + board.count_in_column(column, Camp)
        {
            for row in 0..board.rows.len() {
                if board.grid[(row, column)] == Unassigned {
                    board.grid[(row, column)] = Camp;
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
    fn fill_camps_0_camps() {
        let mut board = Board::new_parse(vec![1, 1, 1], vec![1, 1, 1], "   \n   \n   ").unwrap();
        assert!(!fill_camps(&mut board));
        assert_eq!(board.debug(), "   \n   \n   ");
    }

    #[test]
    fn fill_camps_exact_match() {
        let mut board = Board::new_parse(vec![2, 0, 2], vec![2, 0, 2], " T \nT-T\n T ").unwrap();
        assert!(fill_camps(&mut board));
        assert_eq!(board.debug(), "CTC\nT-T\nCTC");
    }

    #[test]
    fn fill_camps_exact_match_partially_filled() {
        let mut board = Board::new_parse(vec![2, 0, 2], vec![2, 0, 2], "CT \nT-T\n TC").unwrap();
        assert!(fill_camps(&mut board));
        assert_eq!(board.debug(), "CTC\nT-T\nCTC");
    }
}
