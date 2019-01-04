use board::*;
use tile::Tile::*;

/// Fill `Unassigned` slots that can't possibly be `Camp`s with `Grass`.
pub fn initialize_grass(board: &mut Board) -> bool {
    let mut changed = false;
    for row in 0..board.rows.len() {
        for column in 0..board.columns.len() {
            if board.grid[(row, column)] == Unassigned {
                let mut tiles = Vec::new();
                if row + 1 != board.rows.len() {
                    tiles.push(board.grid[(row + 1, column)]);
                }
                if column + 1 != board.columns.len() {
                    tiles.push(board.grid[(row, column + 1)]);
                }
                if row != 0 {
                    tiles.push(board.grid[(row - 1, column)]);
                }
                if column != 0 {
                    tiles.push(board.grid[(row, column - 1)]);
                }
                if tiles.into_iter().all(|x| x != Tree) {
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
    fn initialize_grass_1() {
        let mut board = Board::new_parse(vec![1, 0, 1], vec![2, 0, 0], " T \nT  \n   ").unwrap();
        assert_eq!(board.debug(), " T \nT  \n   ");
        assert!(initialize_grass(&mut board));
        assert_eq!(board.debug(), " T \nT -\n --");
        assert!(!initialize_grass(&mut board));
        assert_eq!(board.debug(), " T \nT -\n --");
    }
}
