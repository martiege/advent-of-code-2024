use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update = Vec::new();

    let mut changed_section = false;

    for line in input.lines() {
        let line = line.trim();
        if !changed_section {
            if line.is_empty() {
                changed_section = true;
                continue;
            }

            if let Some((first, second)) = line.split_once('|') {
                if let (Ok(first), Ok(second)) = (first.parse::<u32>(), second.parse::<u32>()) {
                    if let Some(vector) = page_ordering_rules.get_mut(&first) {
                        vector.push(second);
                    } else {
                        page_ordering_rules.insert(first, vec![second]);
                    }
                }
            }
        } else {
            if line.is_empty() {
                break;
            }

            update.push(line.split(',').filter_map(|x| x.parse().ok()).collect());
        }
    }

    (page_ordering_rules, update)
}

fn update_is_valid(update: &Vec<u32>, orders: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, u) in update.iter().enumerate() {
        if let Some(order) = orders.get(u) {
            for o in order {
                if update[..i].contains(o) {
                    // update is wrong
                    return false;
                }
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (orders, updates) = parse(input);
    let mut sum = 0;

    for update in updates {
        if update_is_valid(&update, &orders) {
            sum += update[update.len() / 2];
        }
    }

    Some(sum)
}

fn sub_reorder_update(update: &Vec<u32>, orders: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut result = update.clone();

    for (i, u) in update.iter().enumerate() {
        if let Some(order) = orders.get(u) {
            for o in order {
                if let Some(p) = result[..i].iter().position(|x| x == o) {
                    result.swap(p, i);
                }
            }
        }
    }

    result
}

fn reorder_update(update: &Vec<u32>, orders: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut result = update.clone();
    while !update_is_valid(&result, orders) {
        result = sub_reorder_update(&result, orders);
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (orders, updates) = parse(input);
    let mut sum = 0;

    for update in updates {
        if update_is_valid(&update, &orders) {
            continue;
        }

        let new_update = reorder_update(&update, &orders);

        sum += new_update[new_update.len() / 2];
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
