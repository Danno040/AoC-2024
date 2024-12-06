use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Default, Clone)]
struct Row {
    pub data: Vec<char>,
}

impl FromIterator<char> for Row {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut c = Row::default();
        let data: &mut Vec<char> = c.data.as_mut();

        for i in iter {
            data.push(i);
        }

        c
    }
}

#[derive(Debug, Default, Clone)]
struct Puzzle {
    pub rows: Vec<Row>,
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownRight,
    Down,
    DownLeft,
}

impl Puzzle {
    fn at(&self, x: usize, y: usize) -> Option<&char> {
        if x >= self.width && y >= self.height {
            return None;
        }

        return match self.rows.get(y) {
            Some(row) => row.data.get(x),
            None => None,
        };
    }

    fn is_char_at(&self, x: usize, y: usize, needle: char) -> bool {
        if let Some(letter) = self.at(x, y) {
            return *letter == needle;
        }

        // No char at x, y
        return false;
    }

    fn is_chain(&self, x: usize, y: usize, chain: Vec<char>, direction: &Direction) -> bool {
        if chain.len() == 0 {
            // Made it to the end! Yay!
            return true;
        }

        if !self.is_char_at(x, y, chain[0]) {
            return false;
        }

        if chain.len() == 1 {
            // We are at the end, since this was the last letter
            return true;
        }

        // Don't overflow with subtractions:
        if x == 0
            && (*direction == Direction::UpLeft
                || *direction == Direction::Left
                || *direction == Direction::DownLeft)
        {
            return false;
        }

        if y == 0
            && (*direction == Direction::UpLeft
                || *direction == Direction::Up
                || *direction == Direction::UpRight)
        {
            return false;
        }

        return match direction {
            Direction::UpLeft => self.is_chain(x - 1, y - 1, chain[1..].to_vec(), direction),
            Direction::Up => self.is_chain(x, y - 1, chain[1..].to_vec(), direction),
            Direction::UpRight => self.is_chain(x + 1, y - 1, chain[1..].to_vec(), direction),
            Direction::Left => self.is_chain(x - 1, y, chain[1..].to_vec(), direction),
            Direction::Right => self.is_chain(x + 1, y, chain[1..].to_vec(), direction),
            Direction::DownRight => self.is_chain(x + 1, y + 1, chain[1..].to_vec(), direction),
            Direction::Down => self.is_chain(x, y + 1, chain[1..].to_vec(), direction),
            Direction::DownLeft => self.is_chain(x - 1, y + 1, chain[1..].to_vec(), direction),
        };
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        // Don't underflow from subtraction:
        if x == 0 || y == 0 {
            return false;
        }

        // Check UpLeft and DownRight for M and S
        let up_left = self.at(x - 1, y - 1).unwrap_or(&'!');
        let down_right = self.at(x + 1, y + 1).unwrap_or(&'!');

        if *up_left == 'M' && *down_right == 'S' {
            // Good
        } else if *up_left == 'S' && *down_right == 'M' {
            // Good
        } else {
            return false;
        }

        // Check UpRight and DownLeft for M and S
        let up_right = self.at(x + 1, y - 1).unwrap_or(&'!');
        let down_left = self.at(x - 1, y + 1).unwrap_or(&'!');

        if *up_right == 'M' && *down_left == 'S' {
            // Good
        } else if *up_right == 'S' && *down_left == 'M' {
            // Good
        } else {
            return false;
        }

        return true;
    }
}

impl FromStr for Puzzle {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Row> = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in s.lines().into_iter() {
            height += 1;
            width = line.len();
            rows.push(line.chars().collect());
        }

        return Ok(Puzzle {
            rows: rows,
            height: height,
            width: width,
        });
    }
}

pub fn process_input(path: &str) {
    // Read in the file, get the two vecs:
    let raw_input = read_to_string(path).expect("Couldn't open day4.txt");
    let puzzle = Puzzle::from_str(&raw_input).expect("Couldn't parse from string");
    println!("X-mases: {}", find_mases(puzzle));
}

pub fn find_xmas(puzzle: Puzzle) -> u64 {
    let mut xmases = 0;

    for y in 0..puzzle.height {
        for x in 0..puzzle.width {
            if puzzle.is_char_at(x, y, 'X') {
                // Look for chains
                let directions = vec![
                    Direction::UpLeft,
                    Direction::Up,
                    Direction::UpRight,
                    Direction::Left,
                    Direction::Right,
                    Direction::DownLeft,
                    Direction::Down,
                    Direction::DownRight,
                ];
                for dir in directions.into_iter() {
                    if puzzle.is_chain(x, y, vec!['X', 'M', 'A', 'S'], &dir) {
                        println!("Found XMAS at ({}, {}) in Dir: {:?}", x, y, dir);
                        xmases += 1;
                    }
                }
            }
        }
    }
    return xmases;
}

pub fn find_mases(puzzle: Puzzle) -> u64 {
    let mut mases = 0;
    for y in 0..puzzle.height {
        for x in 0..puzzle.width {
            if puzzle.is_char_at(x, y, 'A') {
                // Look for MAS
                if puzzle.is_x_mas(x, y) {
                    mases += 1;
                }
            }
        }
    }
    return mases;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_input() {
        let raw_puzzle = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let puzzle = Puzzle::from_str(raw_puzzle).expect("Couldn't parse from string");
        println!("{:?}", puzzle);
        assert_eq!(find_xmas(puzzle), 18);
    }

    #[test]
    fn test_known_input_mases() {
        let raw_puzzle = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let puzzle = Puzzle::from_str(raw_puzzle).expect("Couldn't parse from string");
        println!("{:?}", puzzle);
        assert_eq!(find_mases(puzzle), 9);
    }
}
