use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Unassigned,
    Grass,
    Camp,
    Tree,
}

impl Tile {
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
