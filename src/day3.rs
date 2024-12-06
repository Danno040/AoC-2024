use std::fs::read_to_string;

use regex::Regex;

pub fn process_input(path: &str) {
    // Read in the file, get the two vecs:
    let raw_input = read_to_string(path).expect("Couldn't open day3.txt");
    println!("Result: {}", uncorrupt_with_donts(&raw_input));
}

fn uncorrupt(input: &str) -> i64 {
    let mut result = 0;
    // Find multiplications:
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Couldn't create a regex");

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let left = left.parse::<i64>().expect("Couldn't parse to number");
        let right = right.parse::<i64>().expect("Couldn't parse to number");
        println!("Going to mult {} * {} ", left, right);
        result += left * right;
    }
    return result;
}

fn uncorrupt_with_donts(input: &str) -> i64 {
    let mut result = 0;
    // Split strings by don'ts, then split again on dos, process mults inside
    for (i, line) in input.split("don't").enumerate() {
        if i == 0 {
            // We start enabled
            result += uncorrupt(line);
            continue;
        }

        // Split by dos:
        for (j, inner) in line.split("do()").enumerate() {
            if j == 0 {
                // ignore, this is the don't half
                continue;
            }

            println!("Uncorrupting {}", inner);
            result += uncorrupt(inner);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_input() {
        assert_eq!(
            uncorrupt("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_known_input_with_donts() {
        assert_eq!(
            uncorrupt_with_donts(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
