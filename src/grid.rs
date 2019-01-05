use std::fmt;
use std::ops::{Index, IndexMut};
use tile::Tile::{self, *};

/// A `Grid` of [`Tile`]s.
///
/// [`Tile`]: enum.Tile.html
#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    pub array: Vec<Vec<Tile>>,
}

impl Grid {
    /// Create a new `Grid` from a table of `Tile`s.
    pub fn new(array: Vec<Vec<Tile>>) -> Grid {
        Grid { array }
    }

    /// Create a new `Grid` by parsing the string.
    ///
    /// This parses characters via [`Tile::parse`] and `\n` as the
    /// start of the next row.
    ///
    /// # Examples
    ///
    /// ```
    /// use camps_and_trees::{Grid, Tile::*};
    /// assert_eq!(
    ///    Grid::parse("TC-\n - \n---"),
    ///    Ok(vec![
    ///        vec![Tree, Camp, Grass],
    ///        vec![Unassigned, Grass, Unassigned],
    ///        vec![Grass, Grass, Grass]
    ///    ].into())
    /// );
    /// ```
    pub fn parse(s: &str) -> Result<Grid, String> {
        let mut grid = Vec::new();
        let mut row = Vec::new();
        for c in s.chars() {
            if c == '\n' {
                grid.push(row);
                row = Vec::new();
            } else {
                row.push(Tile::parse(c)?);
            }
        }
        grid.push(row);
        Ok(grid.into())
    }

    /// Create a new blank `Grid` of given dimensions.
    ///
    /// Every element of this `Grid` is [`Unassigned`].
    ///
    /// [`Unassigned`]: enum.Tile.html#variant.Unassigned
    pub fn blank(rows: usize, columns: usize) -> Grid {
        vec![vec![Tile::Unassigned; columns]; rows].into()
    }

    /// Get the `Tile` at `(row, column)`.
    ///
    /// # Errors
    ///
    /// Returns `None` if the coordinates are out of bounds.
    ///
    /// If you are sure the coordinates are in bounds, use the `Index`
    /// operator: `grid[(row, column)]`.
    pub fn get(&self, row: usize, column: usize) -> Option<Tile> {
        self.array.get(row).and_then(|r| r.get(column).cloned())
    }

    /// Set the [`Tile`] at `(row, column)` to a [`Camp`].
    ///
    /// This will fill the surrounding and diagonal tiles with [`Grass`]
    ///
    /// # Errors
    ///
    /// If a [`Camp`] is already at a surrounding or diagonal tile,
    /// then an error is produced.  The `Grid` is not modified on an
    /// error.
    ///
    /// [`Tile`]: enum.Tile.html
    /// [`Camp`]: enum.Tile.html#variant.Camp
    /// [`Grass`]: enum.Tile.html#variant.Grass
    pub fn set_camp(&mut self, row: usize, column: usize) -> Result<(), String> {
        for r in row.saturating_sub(1)..=row + 1 {
            for c in column.saturating_sub(1)..=column + 1 {
                if self.get(r, c) == Some(Camp) {
                    Err(format!(
                        "Camps next to each other at row {}, column {}",
                        row, column
                    ))?;
                }
            }
        }
        self[(row, column)] = Camp;
        for r in row.saturating_sub(1)..=row + 1 {
            for c in column.saturating_sub(1)..=column + 1 {
                if self.get(r, c) == Some(Unassigned) {
                    self[(r, c)] = Grass;
                }
            }
        }
        Ok(())
    }

    /// Get the number of rows in the `Grid`.
    pub fn num_rows(&self) -> usize {
        self.array.len()
    }

    /// Get the number of columns in the `Grid`.
    pub fn num_columns(&self) -> usize {
        self.array.get(0).map(|x| x.len()).unwrap_or(0)
    }

    /// Get the number of `Tile`s equal to `tile` in the given row.
    ///
    /// # Panics
    ///
    /// This will `panic` if `row >= num_rows()`.
    pub fn count_in_row(&self, row: usize, tile: Tile) -> usize {
        // because of the strong guarantees of Vec, this check isn't
        // necessary, but it does make it easier to debug.
        debug_assert!(row < self.num_rows());
        let mut count = 0;
        for column in 0..self.num_columns() {
            if self[(row, column)] == tile {
                count += 1;
            }
        }
        count
    }

    /// Get the number of `Tile`s equal to `tile` in the given column.
    ///
    /// # Panics
    ///
    /// This will `panic` if `column >= num_columns()`.
    pub fn count_in_column(&self, column: usize, tile: Tile) -> usize {
        // because of the strong guarantees of Vec, this check isn't
        // necessary, but it does make it easier to debug.
        debug_assert!(column < self.num_columns());
        let mut count = 0;
        for row in 0..self.num_rows() {
            if self[(row, column)] == tile {
                count += 1;
            }
        }
        count
    }

    /// Get the [`Tile`]s that surround the [`Tile`] at `(row, column)`.
    ///
    /// This will return the points inside the `Grid` with `row +- 1`
    /// *or* `column +- 1`.
    ///
    /// If a [`Camp`] is at `(row, column)`, this will return all
    /// coordinates an associated [`Forest`] could be at.
    ///
    /// # Examples
    ///
    /// Corners will return the two coordinates inside the `Grid`:
    ///
    /// ```
    /// # use camps_and_trees::Grid;
    /// assert_eq!(
    ///     Grid::blank(3, 3).surrounding_tiles(0, 0),
    ///     vec![(0, 1), (1, 0)]
    /// );
    /// ```
    ///
    /// Edges will crop out the coordinate outside them (in this case
    /// `(-1, 1)`):
    ///
    /// ```
    /// # use camps_and_trees::Grid;
    /// assert_eq!(
    ///     Grid::blank(3, 3).surrounding_tiles(0, 1),
    ///     vec![(0, 0), (0, 2), (1, 1)]
    /// );
    /// ```
    ///
    /// Coordinates in the middle will return all four:
    ///
    /// ```
    /// # use camps_and_trees::Grid;
    /// assert_eq!(
    ///     Grid::blank(3, 3).surrounding_tiles(1, 1),
    ///     vec![(0, 1), (1, 0), (1, 2), (2, 1)]
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if `(row, column)` is outside the
    /// `Grid`.
    ///
    /// [`Tile`]: enum.Tile.html
    /// [`Forest`]: enum.Tile.html#variant.Forest
    /// [`Camp`]: enum.Tile.html#variant.Camp
    pub fn surrounding_tiles(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        assert!(self.get(row, column).is_some());
        let mut vec = Vec::new();
        if row != 0 {
            vec.push((row - 1, column));
        }
        if column != 0 {
            vec.push((row, column - 1));
        }
        if column + 1 != self.num_columns() {
            vec.push((row, column + 1));
        }
        if row + 1 != self.num_rows() {
            vec.push((row + 1, column));
        }
        vec
    }

    /// Format the `Grid` in debug mode.
    ///
    /// This is a convenience method similar to `to_string`.
    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }

    /// Is every [`Tile`] not [`Unassigned`]?
    ///
    /// # Remarks
    ///
    /// This method purely tests if the `Grid` has been solved, *not
    /// that it has been solved correctly*.  More advanced analysis is
    /// required for that.
    ///
    /// [`Board::solve`] uses this to determine whether to return an
    /// error or not.  This is because `solve` always correctly solves
    /// `Board`s so long as they didn't start in an incorrect state.
    ///
    /// [`Tile`]: enum.Tile.html
    /// [`Unassigned`]: enum.Tile.html#variant.Unassigned
    /// [`Board::solve`]: struct.Board.html#method.solve
    pub fn is_solved(&self) -> bool {
        self.array
            .iter()
            .all(|row| row.iter().all(|x| *x != Unassigned))
    }
}

impl From<Vec<Vec<Tile>>> for Grid {
    fn from(array: Vec<Vec<Tile>>) -> Grid {
        Grid::new(array)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;
    fn index(&self, index: (usize, usize)) -> &Tile {
        &self.array[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Tile {
        &mut self.array[index.0][index.1]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.array.len() {
            if row != 0 {
                write!(f, "\n")?;
            }
            for x in &self.array[row] {
                write!(f, "{:?}", x)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid_test() {
        assert_eq!(
            Grid::parse("TC-\n - \n---"),
            Ok(vec![
                vec![Tree, Camp, Grass],
                vec![Unassigned, Grass, Unassigned],
                vec![Grass, Grass, Grass]
            ].into())
        );
    }

    #[test]
    fn blank_grid_test() {
        assert_eq!(
            Grid::blank(3, 3),
            vec![
                vec![Unassigned, Unassigned, Unassigned],
                vec![Unassigned, Unassigned, Unassigned],
                vec![Unassigned, Unassigned, Unassigned],
            ].into()
        );
    }

    #[test]
    fn debug_test() {
        assert_eq!(
            "TC-\n - \n---",
            Grid::new(vec![
                vec![Tree, Camp, Grass],
                vec![Unassigned, Grass, Unassigned],
                vec![Grass, Grass, Grass]
            ]).debug()
        );
    }

    #[test]
    fn is_solved_outright_false() {
        assert!(!Grid::parse(" ").unwrap().is_solved());
    }

    #[test]
    fn is_solved_partial() {
        assert!(!Grid::parse("-CT\n---\n T ").unwrap().is_solved());
    }

    #[test]
    fn is_solved_complete() {
        assert!(Grid::parse("-CT\n---\n-T-").unwrap().is_solved());
    }

    #[test]
    fn count_in_row_test() {
        let grid = Grid::parse("C  \nC  \n   ").unwrap();
        assert_eq!(grid.count_in_row(0, Unassigned), 2);
        assert_eq!(grid.count_in_row(0, Grass), 0);
        assert_eq!(grid.count_in_row(0, Camp), 1);
        assert_eq!(grid.count_in_row(0, Tree), 0);
        assert_eq!(grid.count_in_row(1, Camp), 1);
        assert_eq!(grid.count_in_row(2, Camp), 0);
    }

    #[test]
    fn count_in_column_test() {
        let grid = Grid::parse("C  \nC  \n   ").unwrap();
        assert_eq!(grid.count_in_column(0, Unassigned), 1);
        assert_eq!(grid.count_in_column(0, Grass), 0);
        assert_eq!(grid.count_in_column(0, Camp), 2);
        assert_eq!(grid.count_in_column(0, Tree), 0);
        assert_eq!(grid.count_in_column(1, Camp), 0);
        assert_eq!(grid.count_in_column(2, Camp), 0);
    }

    #[test]
    fn surrounding_tiles_corner() {
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(0, 0),
            vec![(0, 1), (1, 0)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(0, 2),
            vec![(0, 1), (1, 2)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(2, 0),
            vec![(1, 0), (2, 1)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(2, 2),
            vec![(1, 2), (2, 1)]
        );
    }

    #[test]
    fn surrounding_tiles_edge() {
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(0, 1),
            vec![(0, 0), (0, 2), (1, 1)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(1, 0),
            vec![(0, 0), (1, 1), (2, 0)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(1, 2),
            vec![(0, 2), (1, 1), (2, 2)]
        );
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(2, 1),
            vec![(1, 1), (2, 0), (2, 2)]
        );
    }

    #[test]
    fn surrounding_tiles_middle() {
        assert_eq!(
            Grid::blank(3, 3).surrounding_tiles(1, 1),
            vec![(0, 1), (1, 0), (1, 2), (2, 1)]
        );
    }

    #[test]
    fn set_camp_test() {
        let mut grid = Grid::parse(" T \nT T\n T ").unwrap();
        assert_eq!(grid.debug(), " T \nT T\n T ");
        assert!(grid.set_camp(0, 0).is_ok());
        assert_eq!(grid.debug(), "CT \nT-T\n T ");
        assert!(grid.set_camp(1, 1).is_err());
        assert_eq!(grid.debug(), "CT \nT-T\n T ");
        assert!(grid.set_camp(0, 2).is_ok());
        assert_eq!(grid.debug(), "CTC\nT-T\n T ");
        assert!(grid.set_camp(1, 1).is_err());
        assert_eq!(grid.debug(), "CTC\nT-T\n T ");
        assert!(grid.set_camp(2, 0).is_ok());
        assert_eq!(grid.debug(), "CTC\nT-T\nCT ");
        assert!(grid.set_camp(2, 0).is_err());
        assert_eq!(grid.debug(), "CTC\nT-T\nCT ");
        assert!(grid.set_camp(2, 2).is_ok());
        assert_eq!(grid.debug(), "CTC\nT-T\nCTC");
    }
}
