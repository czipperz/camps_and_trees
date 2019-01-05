use grid::*;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// The game `Board`.
///
/// This automatically dereferences to the field `grid` for easier
/// usage.
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    /// The number of `Camp`s on every row.
    pub rows: Vec<usize>,
    /// The number of `Camp`s on every column.
    pub columns: Vec<usize>,
    /// The `Grid` of `Tile`s.
    pub grid: Grid,
    marker: PhantomData<()>,
}

impl Board {
    /// Create a new `Board`.
    ///
    /// # Panics
    ///
    /// This will ensure that the [`Grid`] is of a valid size and
    /// `panic` if it isn't.  That is if the length of `rows` is
    /// different than the number of rows in the `grid`, or the same
    /// for `columns`.
    ///
    /// [`Grid`]: struct.Grid.html
    pub fn new(rows: Vec<usize>, columns: Vec<usize>, grid: Grid) -> Self {
        assert_eq!(grid.array.len(), rows.len());
        assert!(grid.array.iter().all(|r| r.len() == rows.len()));
        Board {
            rows,
            columns,
            grid,
            marker: PhantomData,
        }
    }

    /// Create a new `Board` by parsing a string as the [`Grid`].
    ///
    /// This method wraps a call to [`Grid::parse`] and [`Board::new`].
    ///
    /// # Panics
    ///
    /// See [`Board::new`].
    ///
    /// [`Grid`]: struct.Grid.html
    /// [`Grid::parse`]: struct.Grid.html#method.parse
    /// [`Board::new`]: struct.Board.html#method.new
    pub fn new_parse(rows: Vec<usize>, columns: Vec<usize>, s: &str) -> Result<Self, String> {
        Ok(Self::new(rows, columns, Grid::parse(s)?))
    }

    /// Create a new `Board` with a blank [`Grid`] of the correct size.
    ///
    /// This method wraps a call to [`Grid::blank`] and [`Board::new`].
    ///
    /// [`Grid`]: struct.Grid.html
    /// [`Grid::parse`]: struct.Grid.html#method.parse
    /// [`Board::new`]: struct.Board.html#method.new
    pub fn new_blank(rows: Vec<usize>, columns: Vec<usize>) -> Self {
        let grid = Grid::blank(rows.len(), columns.len());
        Self::new(rows, columns, grid)
    }

    /// Solve the `Board` in place.
    ///
    /// # Errors
    ///
    /// If the `Board` cannot be solved automatically, an `Err` is
    /// returned.  The `Board` will be populated with as much
    /// information as can be deduced automatically.
    pub fn solve(&mut self) -> Result<(), String> {
        use associate_trees::*;
        use fill_camps::*;
        use fill_zeros::*;
        use initialize_grass::*;
        use intersection::*;
        initialize_grass(self);
        loop {
            fill_zeros(self);
            if fill_camps(self) {
                continue;
            }
            if process_intersections(self) {
                continue;
            }
            if associate_trees(self) {
                continue;
            }
            break;
        }
        if self.is_solved() {
            Ok(())
        } else {
            Err(format!("Reached steady state\n{:?}", self))
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.grid)
    }
}

impl Deref for Board {
    type Target = Grid;

    fn deref(&self) -> &Grid {
        &self.grid
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tile::Tile::*;

    #[test]
    fn debug_test() {
        assert_eq!(
            "TC-\n - \n---",
            Board::new(
                vec![1, 2, 3],
                vec![3, 2, 1],
                vec![
                    vec![Tree, Camp, Grass],
                    vec![Unassigned, Grass, Unassigned],
                    vec![Grass, Grass, Grass]
                ].into()
            ).debug()
        );
    }

    #[test]
    fn solve_unsolvable() {
        let mut board = Board::new_parse(vec![1, 0, 1], vec![1, 0, 1], " T \n   \n T ").unwrap();
        assert!(board.solve().is_err());
        // but it should make some progress
        assert_eq!(board.debug(), " T \n---\n T ");
    }

    #[test]
    fn solve_5x5_1() {
        let mut board = Board::new_parse(
            vec![1, 1, 0, 2, 1],
            vec![2, 0, 1, 1, 1],
            "     \n T T \n     \nTT   \n    T",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(board.debug(), "---C-\nCT-T-\n-----\nTTC-C\nC---T");
    }

    #[test]
    fn solve_5x5_2() {
        let mut board = Board::new_parse(
            vec![2, 0, 1, 0, 2],
            vec![1, 1, 1, 1, 1],
            " T T \n     \n     \n T   \n TT  ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(board.debug(), "-TCTC\n-----\n-C---\n-T---\nCTTC-");
    }

    #[test]
    fn solve_5x5_10() {
        let mut board = Board::new_parse(
            vec![1, 2, 1, 0, 1],
            vec![2, 0, 1, 1, 1],
            " T   \nT  T \n  T  \n     \n    T",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(board.debug(), "CT---\nT-CTC\nC-T--\n-----\n---CT");
    }

    #[test]
    fn solve_6x6_a5() {
        let mut board = Board::new_parse(
            vec![0, 3, 0, 2, 0, 2],
            vec![1, 1, 2, 1, 0, 2],
            "     T\n   T  \nT     \n  T   \n T    \n   TT ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "-----T\nC-CT-C\nT-----\n-CTC--\n-T----\n--CTTC"
        );
    }

    #[test]
    fn solve_6x6_b10() {
        let mut board = Board::new_parse(
            vec![1, 1, 1, 2, 1, 2],
            vec![2, 1, 2, 0, 1, 2],
            "     T\nT     \n  T   \n     T\nT   T \n T T  ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "----CT\nTC----\n--T--C\nC-C--T\nT---TC\nCTCT--"
        );
    }

    #[test]
    fn solve_7x7_a10() {
        let mut board = Board::new_parse(
            vec![1, 2, 2, 1, 2, 0, 3],
            vec![2, 1, 2, 1, 2, 0, 3],
            "   T   \n  T  T \nT    T \n      T\nT   T  \n   T  T\n T     ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "--CT---\nC-T-CT-\nT-C--TC\n----C-T\nTC--T-C\n---T--T\nCT-C--C"
        );
    }

    #[test]
    fn solve_7x7_b15() {
        let mut board = Board::new_parse(
            vec![2, 1, 2, 1, 2, 1, 2],
            vec![2, 1, 1, 2, 2, 1, 2],
            " T T T \n   T   \nT      \n   T T \nT      \n  T T T\n       ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "-T-TCTC\n-C-T---\nT--C-C-\nC--T-T-\nT--C--C\nC-T-T-T\n--C-C--"
        );
    }

    #[test]
    fn solve_7x7_b20() {
        let mut board = Board::new_parse(
            vec![3, 0, 1, 1, 1, 2, 2],
            vec![2, 1, 2, 1, 1, 2, 1],
            " T  T  \nT      \n    T  \n       \nTT  T T\n       \n   T T ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "CTC-TC-\nT------\n----TC-\n-C-----\nTT-CT-T\nC-----C\n--CTCT-"
        );
    }

    #[test]
    fn solve_8x8_b2() {
        let mut board = Board::new_parse(
            vec![2, 1, 2, 1, 3, 1, 1, 3],
            vec![3, 1, 1, 2, 2, 1, 2, 2],
            "  T     \nT    T T\n     T  \n    T   \n        \nT T  T T\n  T T   \n T    T ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "-CT----C\nT---CT-T\nC----TC-\n---CT---\nC----C-C\nT-TC-T-T\n--T-T-C-\nCTC-C-T-"
        );
    }

    #[test]
    fn solve_8x8_b5() {
        let mut board = Board::new_parse(
            vec![1, 3, 1, 3, 1, 2, 1, 3],
            vec![4, 0, 3, 1, 1, 2, 2, 2],
            "T  T    \n     T  \nT    TTT\n  T     \nT       \n  T  T  \n  T    T\n T    T ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "T-CT----\nC---CTC-\nT-C--TTT\nC-T--C-C\nT-C-----\nC-T--TC-\n--TC---T\nCT---CTC"
        );
    }

    #[test]
    fn solve_8x8_b9() {
        let mut board = Board::new_parse(
            vec![1, 1, 1, 2, 2, 1, 2, 2],
            vec![1, 1, 2, 1, 1, 3, 1, 2],
            "        \n T   T  \n    TT  \n       T\nT  T    \n        \n  TT T  \n    T  T",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "-----C--\n-TC--T--\n----TTC-\nC---C--T\nT-CT---C\n-----C--\n-CTT-T-C\n---CTC-T"
        );
    }

    #[test]
    fn solve_8x8_b10() {
        let mut board = Board::new_parse(
            vec![2, 2, 2, 0, 3, 0, 3, 1],
            vec![1, 3, 1, 2, 1, 2, 0, 3],
            "  T T  T\n    T   \n        \n T T    \n      T \n TT    T\n     T  \nT     T ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "-CTCT--T\n----TC-C\n-C-C----\n-T-T----\n-C---CTC\n-TT----T\nC-C-CT--\nT-----TC"
        );
    }

    #[test]
    fn solve_8x8_b13() {
        let mut board = Board::new_parse(
            vec![3, 1, 2, 1, 1, 2, 1, 3],
            vec![4, 0, 2, 1, 3, 1, 1, 2],
            "       T\nT  TTT  \n T      \n        \n  T T   \n T     T\n        \n T T TT ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "C--C-C-T\nT--TTT-C\nCT--C---\n--C-----\n--T-T--C\nCT--C--T\n------C-\nCTCTCTT-"
        );
    }

    #[test]
    fn solve_8x8_b15() {
        let mut board = Board::new_parse(
            vec![2, 1, 2, 1, 2, 2, 1, 1],
            vec![2, 2, 1, 2, 0, 3, 1, 1],
            "T      T\n   T    \n        \n  T  T T\nT   T   \n  T  T  \n T T    \n        ",
        ).unwrap();
        board.solve().unwrap();
        assert_eq!(
            board.debug(),
            "T--C--CT\nC--T----\n--C--C--\nC-T--T-T\nT---TC-C\n-CTC-T--\n-T-T-C--\n-C------"
        );
    }
}
