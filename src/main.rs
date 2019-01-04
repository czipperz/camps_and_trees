pub mod associate_trees;
pub mod board;
pub mod fill_camps;
pub mod fill_zeros;
pub mod grid;
pub mod initialize_grass;
pub mod intersection;
pub mod tile;
use board::Board;

fn read_camps(s: &str) -> Result<Vec<usize>, String> {
    if s.is_empty() {
        Err("Row or column descriptors must not be empty")?
    }
    let camps: Result<_, _> = s.split(',').map(|x| x.trim()).map(|x| x.parse()).collect();
    camps.map_err(|x: std::num::ParseIntError| x.to_string())
}

pub fn analyze_stdin(lines: Vec<String>) -> Result<Board, String> {
    if lines.len() < 3 {
        Err("Too few lines.  There must be at least 3.")?
    }
    let rows = read_camps(&lines[0])?;
    let columns = read_camps(&lines[1])?;
    Board::new_parse(rows, columns, &lines[2..].join("\n"))
}

fn get_stdin_lines() -> Result<Vec<String>, String> {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let lines: Result<_, _> = stdin.lock().lines().collect();
    lines.map_err(|x| x.to_string())
}

fn try_main() -> Result<(), String> {
    let mut board = analyze_stdin(get_stdin_lines()?)?;
    board.solve()?;
    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_stdin_too_little_input() {
        assert!(analyze_stdin(vec![]).is_err());
        assert!(analyze_stdin(vec!["0".to_string(), "0".to_string()]).is_err());
    }

    #[test]
    fn analyze_stdin_2x2() {
        assert_eq!(
            analyze_stdin(vec![
                "1, 0".to_string(),
                "1, 0".to_string(),
                " T".to_string(),
                "  ".to_string()
            ]),
            Ok(Board::new_parse(vec![1, 0], vec![1, 0], " T\n  ").unwrap())
        );
    }

    #[test]
    fn read_camps_empty() {
        assert!(read_camps("").is_err());
    }

    #[test]
    fn read_camps_one_element() {
        assert_eq!(read_camps("1"), Ok(vec![1]));
    }

    #[test]
    fn read_camps_three_elements() {
        assert_eq!(read_camps("1, 2, 3"), Ok(vec![1, 2, 3]));
    }
}
