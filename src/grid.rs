use std::fmt;
use std::ops::{Index, IndexMut};
use tile::Tile::{self, *};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    pub array: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(array: Vec<Vec<Tile>>) -> Grid {
        Grid { array }
    }

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

    pub fn blank(rows: usize, columns: usize) -> Grid {
        vec![vec![Tile::Unassigned; columns]; rows].into()
    }

    pub fn get(&self, row: usize, column: usize) -> Option<Tile> {
        self.array.get(row).and_then(|r| r.get(column).cloned())
    }

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

    pub fn num_rows(&self) -> usize {
        self.array.len()
    }

    pub fn num_columns(&self) -> usize {
        self.array.get(0).map(|x| x.len()).unwrap_or(0)
    }

    pub fn count_in_row(&self, row: usize, tile: Tile) -> usize {
        let mut count = 0;
        for column in 0..self.num_columns() {
            if self[(row, column)] == tile {
                count += 1;
            }
        }
        count
    }

    pub fn count_in_column(&self, column: usize, tile: Tile) -> usize {
        let mut count = 0;
        for row in 0..self.num_rows() {
            if self[(row, column)] == tile {
                count += 1;
            }
        }
        count
    }

    pub fn surrounding_tiles(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();
        if row != 0 {
            vec.push((row - 1, column));
        }
        if row + 1 != self.num_rows() {
            vec.push((row + 1, column));
        }
        if column != 0 {
            vec.push((row, column - 1));
        }
        if column + 1 != self.num_columns() {
            vec.push((row, column + 1));
        }
        vec
    }

    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }

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
