use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Heading {
    Up,    // ^
    Down,  // v
    Left,  // >
    Right, // <
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Spot {
    Empty,
    Blocked,
    Guard(Heading),
    Visited(Heading),
}

type Row = Vec<Spot>;
type Map = Vec<Row>;

trait Mappable {
    fn from_str(value: &str) -> Map;

    fn at(&self, x: usize, y: usize) -> Option<&Spot>;

    fn find_guard(&self) -> (usize, usize, &Heading);

    fn count_visited(&self) -> usize;

    fn to_str(&self) -> String;
}

impl Mappable for Map {
    fn from_str(value: &str) -> Map {
        let mut new_map: Map = Vec::new();
        for line in value.lines().into_iter() {
            let mut new_row: Row = Vec::new();
            for letter in line.chars().into_iter() {
                match letter {
                    '.' => new_row.push(Spot::Empty),
                    '#' => new_row.push(Spot::Blocked),
                    '^' => new_row.push(Spot::Guard(Heading::Up)),
                    'v' => new_row.push(Spot::Guard(Heading::Down)),
                    '<' => new_row.push(Spot::Guard(Heading::Left)),
                    '>' => new_row.push(Spot::Guard(Heading::Right)),
                    _ => panic!("Got an invalid character"),
                };
            }
            new_map.push(new_row);
        }

        return new_map;
    }

    fn at(&self, x: usize, y: usize) -> Option<&Spot> {
        if y >= self.len() || x >= self[y].len() {
            return None;
        }
        return Some(&self[y][x]);
    }

    fn find_guard(&self) -> (usize, usize, &Heading) {
        for (y, row) in self.into_iter().enumerate() {
            for (x, spot) in row.into_iter().enumerate() {
                match spot {
                    Spot::Guard(heading) => return (x, y, heading),
                    _ => {} // Ignore everything else
                }
            }
        }

        panic!("Coudn't find a guard!");
    }

    fn count_visited(&self) -> usize {
        let mut total = 0;

        for (y, row) in self.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                match spot {
                    Spot::Visited(_) => total += 1,
                    _ => {} // Ignore everything else
                }
            }
        }
        return total;
    }

    fn to_str(&self) -> String {
        let mut result = String::new();
        for (y, row) in self.into_iter().enumerate() {
            for (x, spot) in row.into_iter().enumerate() {
                result += match spot {
                    Spot::Empty => ".",
                    Spot::Blocked => "#",
                    Spot::Guard(_) => "$",
                    Spot::Visited(heading) => match heading {
                        Heading::Up => "^",
                        Heading::Down => "v",
                        Heading::Left => "<",
                        Heading::Right => ">",
                    },
                }
            }
            result += "\n";
        }

        return result;
    }
}

pub fn walk(map: &Map, start: (usize, usize), direction: &Heading) -> Map {
    let mut new_map = map.clone();
    let mut current_pos = start;
    let mut current_direction = direction;
    loop {
        // Make sure we don't underflow:
        if current_pos.0 == 0 && *current_direction == Heading::Left {
            new_map[current_pos.1][current_pos.0] = Spot::Visited(Heading::Left);
            break;
        } else if current_pos.1 == 0 && *current_direction == Heading::Up {
            new_map[current_pos.1][current_pos.0] = Spot::Visited(Heading::Up);
            break;
        }

        // Check next step
        let to_check = match current_direction {
            Heading::Up => (current_pos.0, current_pos.1 - 1),
            Heading::Down => (current_pos.0, current_pos.1 + 1),
            Heading::Left => (current_pos.0 - 1, current_pos.1),
            Heading::Right => (current_pos.0 + 1, current_pos.1),
        };

        let next_spot = new_map.at(to_check.0, to_check.1);
        match next_spot {
            Some(spot) => {
                match spot {
                    Spot::Empty => {
                        new_map[current_pos.1][current_pos.0] =
                            Spot::Visited(current_direction.clone());
                        current_pos = to_check;
                    }
                    Spot::Blocked => {
                        new_map[current_pos.1][current_pos.0] =
                            Spot::Visited(current_direction.clone());
                        // Need to change direction
                        match current_direction {
                            Heading::Up => {
                                current_direction = &Heading::Right;
                            }
                            Heading::Down => {
                                current_direction = &Heading::Left;
                            }
                            Heading::Left => {
                                current_direction = &Heading::Up;
                            }
                            Heading::Right => {
                                current_direction = &Heading::Down;
                            }
                        }
                    }
                    Spot::Guard(_) => panic!("Ran into the guard!"),
                    Spot::Visited(_) => {
                        new_map[current_pos.1][current_pos.0] =
                            Spot::Visited(current_direction.clone());
                        // Already been here, just keep rolling
                        current_pos = to_check;
                    }
                }
            }
            None => {
                new_map[current_pos.1][current_pos.0] = Spot::Visited(current_direction.clone());
                break;
            }
        }
    }

    return new_map;
}

fn count_possible_obstruction_positions(
    map: &Map,
    start: (usize, usize),
    direction: &Heading,
) -> usize {
    // This will be like walk, but see if blocking at each step would result in pointing the same direction again
    let mut count = 0;
    let mut current_pos = start;
    let mut current_direction = direction;
    loop {
        // Make sure we don't underflow:
        if current_pos.0 == 0 && *current_direction == Heading::Left {
            break;
        } else if current_pos.1 == 0 && *current_direction == Heading::Up {
            break;
        }

        // Check next step
        let to_check = match current_direction {
            Heading::Up => (current_pos.0, current_pos.1 - 1),
            Heading::Down => (current_pos.0, current_pos.1 + 1),
            Heading::Left => (current_pos.0 - 1, current_pos.1),
            Heading::Right => (current_pos.0 + 1, current_pos.1),
        };

        let next_spot = map.at(to_check.0, to_check.1);
        println!(
            "Checking ({}, {}) heading {:?}",
            to_check.0, to_check.1, current_direction
        );
        match next_spot {
            Some(spot) => {
                match spot {
                    Spot::Empty => {
                        // Should never happen
                        panic!("Found unvisited node");
                    }
                    Spot::Blocked => {
                        // Need to change direction
                        match current_direction {
                            Heading::Up => {
                                current_direction = &Heading::Right;
                            }
                            Heading::Down => {
                                current_direction = &Heading::Left;
                            }
                            Heading::Left => {
                                current_direction = &Heading::Up;
                            }
                            Heading::Right => {
                                current_direction = &Heading::Down;
                            }
                        }
                    }
                    Spot::Guard(_) => panic!("Ran into the guard!"),
                    Spot::Visited(_) => {
                        // What if this _wasnt_ empty, would turning here put me in a loop?
                        match current_direction {
                            Heading::Up => {
                                let next_if_turned = map.at(current_pos.0 + 1, current_pos.1);
                                if next_if_turned.is_some()
                                    && *next_if_turned.unwrap() == Spot::Visited(Heading::Right)
                                {
                                    count += 1;
                                }
                            }
                            Heading::Down => {
                                let next_if_turned = map.at(current_pos.0 - 1, current_pos.1);
                                if next_if_turned.is_some()
                                    && *next_if_turned.unwrap() == Spot::Visited(Heading::Left)
                                {
                                    count += 1;
                                }
                            }
                            Heading::Left => {
                                let next_if_turned = map.at(current_pos.0, current_pos.1 - 1);
                                if next_if_turned.is_some()
                                    && *next_if_turned.unwrap() == Spot::Visited(Heading::Up)
                                {
                                    count += 1;
                                }
                            }
                            Heading::Right => {
                                let next_if_turned = map.at(current_pos.0, current_pos.1 + 1);
                                if next_if_turned.is_some()
                                    && *next_if_turned.unwrap() == Spot::Visited(Heading::Down)
                                {
                                    count += 1;
                                }
                                current_direction = &Heading::Down;
                            }
                        }
                        current_pos = to_check;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    return count;
}

pub fn process_input(path: &str) {
    let raw_input = read_to_string(path).expect("Couldn't open day6.txt");
    let map = Map::from_str(&raw_input);

    let guard_info = map.find_guard();
    let new_map = walk(&map, (guard_info.0, guard_info.1), guard_info.2);

    let final_count = new_map.count_visited();
    println!("Day 6 part 1: {}", final_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_input_to_map() {
        let raw_input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let mut map = Map::from_str(&raw_input);

        assert_eq!(
            map,
            vec![
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Guard(Heading::Up),
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty
                ],
                vec![
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
                vec![
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Blocked,
                    Spot::Empty,
                    Spot::Empty,
                    Spot::Empty
                ],
            ]
        );

        let guard_info = map.find_guard();
        let expected: (usize, usize, &Heading) = (4, 6, &Heading::Up);
        assert_eq!(guard_info, expected);

        let new_map = walk(&map, (guard_info.0, guard_info.1), guard_info.2);

        let final_count = new_map.count_visited();
        assert_eq!(final_count, 41);
    }

    #[test]
    fn test_input_with_new_obstructions() {
        let raw_input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let mut map = Map::from_str(&raw_input);
        let guard_info = map.find_guard();
        let new_map = walk(&map, (guard_info.0, guard_info.1), guard_info.2);

        println!("{}", new_map.to_str());
        let final_count = count_possible_obstruction_positions(
            &new_map,
            (guard_info.0, guard_info.1),
            guard_info.2,
        );
        assert_eq!(final_count, 6);
    }
}
