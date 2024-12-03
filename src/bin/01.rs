use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

fn extract(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .filter_map(
            |(a, b)| match (a.trim().parse::<i64>().ok(), b.trim().parse::<i64>().ok()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            },
        )
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = extract(input);

    list1.sort();
    list2.sort();

    Some(
        zip::<_, _>(list1, list2)
            .map(|(c1, c2)| (c1 - c2).unsigned_abs() as u32)
            .sum(),
    )
}

fn count_element_function<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = extract(input);

    let counts = count_element_function(list2);

    Some(list1.iter().fold(0, |acc, x| {
        acc + (if counts.contains_key(x) {
            (counts[x] * (*x as usize)) as u32
        } else {
            0
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
