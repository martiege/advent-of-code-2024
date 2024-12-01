use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = input
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .filter_map(
            |(a, b)| match (a.trim().parse::<i64>().ok(), b.trim().parse::<i64>().ok()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            },
        )
        .unzip();

    list1.sort();
    list2.sort();

    Some(
        zip::<_, _>(list1, list2)
            .map(|(c1, c2)| (c1 - c2).abs() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
