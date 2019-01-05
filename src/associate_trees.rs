use grid::*;
use tile::Tile::*;

/// The association of a certain `Tile`.
///
/// This tells us if there is a tree
#[derive(Clone, PartialEq, Eq, Debug)]
enum Association {
    /// This `Tile` is a `Tree` with an associated `Camp` at `(row, column)`.
    CampAt(usize, usize),
    /// This `Tile` is a `Tree` with no associated `Camp`.
    NoCampAssociated,
    /// This `Tile` is a `Camp` with no associated `Tree`.
    UnassignedCamp,
    /// This `Tile` is a `Camp` with an associated `Tree` or `Grass` or `Unspecified`.
    NoTree,
    /// This `Tile` has not yet been processed.
    Unprocessed,
}
use self::Association::*;

impl Association {
    /// True if `self` is a `CampAt`.
    fn is_camp_at(&self) -> bool {
        match self {
            CampAt(_, _) => true,
            _ => false,
        }
    }
}

/// Call `associate_tree` for each `surrounding_tile`.
fn associate_surrounding_trees(
    grid: &Grid,
    row: usize,
    column: usize,
    associations: &mut Vec<Vec<Association>>,
) {
    for (r, c) in grid.surrounding_tiles(row, column) {
        associate_tree(grid, r, c, associations);
    }
}

/// Populate the `associations` table for the `Tile` at `(row, column)`.
fn associate_tree(
    grid: &Grid,
    row: usize,
    column: usize,
    associations: &mut Vec<Vec<Association>>,
) {
    if associations[row][column] == Unprocessed {
        if grid[(row, column)] == Tree {
            associations[row][column] = NoCampAssociated;
            associate_surrounding_trees(grid, row, column, associations);
        } else if grid[(row, column)] == Camp {
            associations[row][column] = UnassignedCamp;
            associate_surrounding_trees(grid, row, column, associations);
            // `Camp` handles assigning itself to `Tree`s around it.
            let trees: Vec<_> = grid
                .surrounding_tiles(row, column)
                .into_iter()
                .filter(|&p| grid[p] == Tree)
                .collect();
            assert!(trees.len() >= 1);
            assert!(trees.len() <= 4);
            // If there is exactly one Tree next to this Camp, then we
            // associate ourselves with it.  Otherwise it can be
            // ambiguous.
            if trees.len() == 1 {
                let (r, c) = trees[0];
                assert_eq!(associations[r][c], NoCampAssociated);
                associations[r][c] = CampAt(row, column);
                associations[row][column] = NoTree;
            }
        } else {
            associations[row][column] = NoTree;
        }
    }
}

/// Generate the initial associations table.
fn generate_associations(rows: usize, columns: usize) -> Vec<Vec<Association>> {
    vec![vec![Association::Unprocessed; columns]; rows]
}

/// Associate [`Tree`]s with [`Camp`]s and fill in [`Grass`] around
/// booked [`Tree`]s.
///
/// # Examples
///
/// This resolves the problem of a [`Camp`] being placed next to a
/// [`Tree`] without the space on the other side being turned to
/// [`Grass`]:
///
/// ```
/// # use camps_and_trees::{Grid, associate_trees};
/// let mut grid = Grid::parse("---\n TC\n---").unwrap();
/// associate_trees(&mut grid);
/// assert_eq!(grid, Grid::parse("---\n-TC\n---").unwrap());
/// ```
///
/// However it also acts conservatively, not filling in [`Grass`] unless
/// it can show it to be needed:
///
/// ```
/// # use camps_and_trees::{Grid, associate_trees};
/// let mut grid = Grid::parse("T--\n TC\nT--").unwrap();
/// associate_trees(&mut grid);
/// assert_eq!(grid, Grid::parse("T--\n TC\nT--").unwrap());
/// ```
///
/// [`Tree`]: enum.Tile.html#variant.Tree
/// [`Camp`]: enum.Tile.html#variant.Camp
/// [`Grass`]: enum.Tile.html#variant.Grass
pub fn associate_trees(grid: &mut Grid) -> bool {
    let mut changed = false;
    let mut associations: Vec<Vec<Association>> =
        generate_associations(grid.num_rows(), grid.num_columns());
    for row in 0..grid.num_rows() {
        for column in 0..grid.num_columns() {
            associate_tree(grid, row, column, &mut associations);
        }
    }
    for row in 0..grid.num_rows() {
        for column in 0..grid.num_columns() {
            if grid[(row, column)] == Unassigned {
                if grid
                    .surrounding_tiles(row, column)
                    .into_iter()
                    .all(|x| grid[x] != Tree || associations[x.0][x.1].is_camp_at())
                {
                    grid[(row, column)] = Grass;
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
    fn associate_tree_no_camp() {
        let grid = Grid::parse(" T \n   \n   ").unwrap();
        let mut associations = generate_associations(3, 3);
        associate_tree(&grid, 0, 1, &mut associations);
        assert_eq!(
            associations,
            vec![
                vec![NoTree, NoCampAssociated, NoTree],
                vec![Unprocessed, NoTree, Unprocessed],
                vec![Unprocessed, Unprocessed, Unprocessed],
            ]
        );
    }

    #[test]
    fn associate_tree_associate_tree() {
        let grid = Grid::parse(" TC\n --\n   ").unwrap();
        let mut associations = generate_associations(3, 3);
        associate_tree(&grid, 0, 1, &mut associations);
        assert_eq!(
            associations,
            vec![
                vec![NoTree, CampAt(0, 2), NoTree],
                vec![Unprocessed, NoTree, NoTree],
                vec![Unprocessed, Unprocessed, Unprocessed],
            ]
        );
    }

    #[test]
    fn associate_trees_horizontal() {
        let mut grid = Grid::parse(" TC\n---\n---").unwrap();
        assert!(associate_trees(&mut grid));
        assert_eq!(grid.debug(), "-TC\n---\n---");
    }
}
