use std::{collections::HashMap, fs::read_to_string};

pub fn process_input(path: &str) {
    let mut left = Vec::<u64>::new();
    let mut right = Vec::<u64>::new();

    // Read in the file, get the two vecs:
    let raw_input = read_to_string(path).expect("Couldn't open day1.txt");
    for line in raw_input.split('\n').into_iter() {
        if line.len() == 0 {
            continue;
        }
        let split: Vec<&str> = line.split_whitespace().collect();
        left.push(split[0].parse::<u64>().expect("Couldnt' parse to number"));
        right.push(split[1].parse::<u64>().expect("Couldnt' parse to number"));
    }

    left.sort_unstable();
    right.sort_unstable();

    println!("{}", calculate_similarity(left, right));
}

pub fn calculate_distances(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort_unstable();
    right.sort_unstable();

    assert!(left.len() == right.len());
    let mut result: u64 = 0;

    for (i, a) in left.into_iter().enumerate() {
        result += a.abs_diff(right[i]);
    }

    return result;
}

pub fn calculate_similarity(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort_unstable();
    right.sort_unstable();

    let mut dup_map = HashMap::new();

    assert!(left.len() == right.len());
    let mut result: u64 = 0;

    for a in left.into_iter() {
        loop {
            if right.len() > 0 && a > right[0] {
                right.remove(0);
            } else {
                break;
            }
        }

        // Count how many times a is in right:
        let dups = dup_map.entry(a).or_insert_with(|| {
            let mut dups: u64 = 0;
            loop {
                if right.len() == 0 {
                    break;
                } else if a == right[0] {
                    dups += 1;
                    right.remove(0);
                } else {
                    break;
                }
            }
            return dups;
        });

        result += a * (*dups);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distances() {
        let left: Vec<u64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<u64> = vec![4, 3, 5, 3, 9, 3];

        let result = calculate_distances(left, right);
        assert_eq!(result, 11)
    }

    #[test]
    fn test_calculate_similarity() {
        let left: Vec<u64> = vec![3, 4, 2, 1, 3, 3];
        let right: Vec<u64> = vec![4, 3, 5, 3, 9, 3];

        let result = calculate_similarity(left, right);
        assert_eq!(result, 31)
    }

    #[test]
    fn test_calculate_similarity_where_mismatch() {
        let left: Vec<u64> = vec![3, 4, 2, 2, 3, 3];
        let right: Vec<u64> = vec![4, 3, 5, 3, 9, 1];
        let result = calculate_similarity(left, right);
        assert_eq!(result, 22)
    }
}
