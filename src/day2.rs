use std::fs::read_to_string;

pub fn process_input(path: &str) {
    let mut safe_reports = 0;

    // Read in the file, get the two vecs:
    let raw_input = read_to_string(path).expect("Couldn't open day2.txt");
    for line in raw_input.split('\n').into_iter() {
        if line.len() == 0 {
            continue;
        }
        let mut report = Vec::new();
        for value in line.split_whitespace().into_iter() {
            report.push(value.parse::<u64>().expect("Couldn't parse to number"));
        }

        if is_safe_with_dampener(report) {
            safe_reports += 1;
        }
    }

    println!("Safe: {}", safe_reports);
}

fn is_safe(mut report: Vec<u64>) -> bool {
    assert!(report.len() > 1);

    let is_increasing = report[0] < report[1];
    let diff = report[0].abs_diff(report[1]);
    if diff < 1 || diff > 3 {
        return false;
    }
    report.remove(0);
    let mut prev = report[0];
    report.remove(0);
    for next in report.into_iter() {
        if is_increasing && prev > next {
            return false;
        } else if !is_increasing && prev < next {
            return false;
        }
        let diff = next.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false;
        }
        prev = next;
    }

    return true;
}

fn is_safe_with_dampener(mut report: Vec<u64>) -> bool {
    // Is it safe _without_ modifications?
    if is_safe(report.clone()) {
        return true;
    }

    for (i, _value) in report.iter().enumerate() {
        // Is it safe without one of the numbers?
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(modified_report) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_is_safe() {
        assert_eq!(is_safe(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_report_is_safe_with_dampener() {
        assert_eq!(is_safe_with_dampener(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe_with_dampener(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe_with_dampener(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe_with_dampener(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(is_safe_with_dampener(vec![8, 6, 4, 4, 1]), true);
        assert_eq!(is_safe_with_dampener(vec![1, 3, 6, 7, 9]), true);
        assert_eq!(is_safe_with_dampener(vec![2, 1, 4, 5, 6]), true);
    }
}
