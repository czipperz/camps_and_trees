use std::fmt;

/// A single `Tile` on the [`Grid`].
///
/// [`Grid`]: struct.Grid.html
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    /// This `Tile` has not yet been assigned or solved for.
    Unassigned,
    Grass,
    Camp,
    Tree,
}

impl Tile {
    /// Parse the char into a `Tile`.
    ///
    /// ` ` is `Unassigned`, `-` is `Grass`, `C` is `Camp`, and `T` is `Tree`.
    ///
    /// # Errors
    ///
    /// If the char doesn't match one of the four options outlined
    /// above, an `Err` is returned.
    pub fn parse(c: char) -> Result<Self, String> {
        match c {
            ' ' => Ok(Tile::Unassigned),
            '-' => Ok(Tile::Grass),
            'C' => Ok(Tile::Camp),
            'T' => Ok(Tile::Tree),
            _ => Err(format!("Couldn't parse tile: '{}'", c)),
        }
    }
}

impl fmt::Debug for Tile {
    /// See the method [`parse`].
    ///
    /// [`parse`]: enum.Tile.html#method.parse
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Unassigned => ' ',
                Tile::Grass => '-',
                Tile::Camp => 'C',
                Tile::Tree => 'T',
            }
        )
    }
}
