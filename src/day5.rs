use std::{collections::HashMap, fs::read_to_string};

type Rules = HashMap<u64, Vec<u64>>;
type Order = Vec<u64>;

trait Findable {
    fn find(&self, value: u64) -> Option<usize>;
}

impl Findable for Order {
    fn find(&self, value: u64) -> Option<usize> {
        for (i, v) in self.into_iter().enumerate() {
            if *v == value {
                return Some(i);
            }
        }
        return None;
    }
}

fn is_order_valid(rules: &Rules, order: &Order) -> bool {
    let mut already_checked: Order = Vec::new();

    for page in order.into_iter() {
        if let Some(page_rule) = rules.get(page) {
            for r in page_rule.into_iter() {
                if already_checked.contains(r) {
                    return false;
                }
            }
        }
        already_checked.push(*page);
    }

    return true;
}

fn get_middle_page(order: &Order) -> u64 {
    return *order.get(order.len() / 2).expect("couldn't get middle");
}

pub fn process_input(path: &str) {
    // Read in the file, get the two vecs:
    let raw_input = read_to_string(path).expect("Couldn't open day5.txt");
    let (rules, orders) = raw_input_to_rules_and_orders(&raw_input);
    // let result = orders
    //     .into_iter()
    //     .filter(|order| is_order_valid(&rules, order))
    //     .map(|order| get_middle_page(&order))
    //     .reduce(|acc, page| {
    //         return acc + page;
    //     })
    //     .expect("Didn't reduce correctly");

    // println!("Day 5 result: {}", result);

    let invalid_result = orders
        .into_iter()
        .filter(|order| !is_order_valid(&rules, order))
        .map(|order| correct_invalid_order(&rules, &order))
        .map(|order| get_middle_page(&order))
        .reduce(|acc, page| {
            return acc + page;
        })
        .expect("Didn't reduce correctly");
    println!("Day 5 part 2 result: {}", invalid_result);
}

fn raw_input_to_rules_and_orders(raw_input: &str) -> (Rules, Vec<Order>) {
    let mut rules: Rules = HashMap::new();
    let mut orders: Vec<Order> = Vec::new();
    let (raw_rules, raw_orders) = raw_input
        .split_once("\n\n")
        .expect("Couldn't find the space between");

    for line in raw_rules.lines().into_iter() {
        let (left, right) = line.split_once('|').expect("Couldn't split on |");
        let left = left.parse::<u64>().expect("Couldnt' parse as number");
        let right = right.parse::<u64>().expect("Couldnt' parse as number");

        rules
            .entry(left)
            .and_modify(|e| e.push(right))
            .or_insert_with(|| {
                return vec![right];
            });
    }

    for line in raw_orders.lines().into_iter() {
        orders.push(
            line.split(',')
                .map(|x| x.parse::<u64>().expect("couldn't parse number"))
                .collect(),
        );
    }

    return (rules, orders);
}

fn correct_invalid_order(rules: &Rules, invalid_order: &Order) -> Order {
    // Like a normal check, but switch pages when we hit a rule violation
    let mut already_checked: Order = Vec::new();

    for page in invalid_order.into_iter() {
        if let Some(page_rule) = rules.get(page) {
            for r in page_rule.into_iter() {
                if already_checked.contains(r) {
                    // Swap places
                    already_checked.push(*page);
                    let j = already_checked.len() - 1;
                    let i = already_checked.find(*r).expect("Couldn't find index");
                    already_checked.swap(i, j);
                    break;
                }
            }
        }
        if !already_checked.contains(page) {
            already_checked.push(*page);
        }
    }

    if !is_order_valid(rules, &already_checked) {
        return correct_invalid_order(rules, &already_checked);
    }

    return already_checked;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_input() {
        let raw_input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let (rules, orders) = raw_input_to_rules_and_orders(raw_input);
        println!("{:?}\n\n{:?}", rules, orders);

        let valid_orders: Vec<Vec<u64>> = orders
            .into_iter()
            .filter(|order| is_order_valid(&rules, order))
            .collect();
        assert_eq!(
            valid_orders,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ]
        );

        let middle_pages: Vec<u64> = valid_orders
            .into_iter()
            .map(|order| get_middle_page(&order))
            .collect();
        assert_eq!(middle_pages, vec![61, 53, 29]);
    }

    #[test]
    fn test_known_input_part_2() {
        let raw_input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        let (rules, orders) = raw_input_to_rules_and_orders(raw_input);

        let invalid_orders: Vec<Vec<u64>> = orders
            .into_iter()
            .filter(|order| !is_order_valid(&rules, order))
            .collect();
        assert_eq!(
            invalid_orders,
            vec![
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        );

        let middle_pages: Vec<u64> = invalid_orders
            .into_iter()
            .map(|order| correct_invalid_order(&rules, &order))
            .map(|order| get_middle_page(&order))
            .collect();
        assert_eq!(middle_pages, vec![47, 29, 47]);
    }
}
