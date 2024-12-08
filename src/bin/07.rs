use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

fn parse(input: &str) -> IResult<&str, Vec<(u32, Vec<u32>)>> {
    separated_list1(
        tag("\n"),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            tag(": "),
            separated_list0(tag(" "), map_res(digit1, |s: &str| s.parse::<u32>())),
        ),
    )(input)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Operations {
    Plus,
    Mult,
}

fn operations_folder(acc: Option<u32>, data: &((&u32, &u32), Operations)) -> Option<u32> {
    match (acc, data) {
        (None, ((a, b), Operations::Plus)) => Some(*a + *b),
        (None, ((a, b), Operations::Mult)) => Some(*a * *b),
        (Some(acc), ((_, b), Operations::Plus)) => Some(acc + *b),
        (Some(acc), ((_, b), Operations::Mult)) => Some(acc * *b),
    }
}

fn get_all_operations<'a, T>(
    data: &'a [(T, T)],
) -> impl Iterator<Item = Vec<((T, T), Operations)>> + 'a
where
    T: Copy + 'a,
{
    [Operations::Plus, Operations::Mult]
        .iter()
        .combinations_with_replacement(data.len())
        .flat_map(|v| v.into_iter().permutations(data.len()))
        .unique()
        .map(|x| {
            data.iter()
                .zip(x)
                .map(|(n, c)| (*n, c.clone()))
                .collect::<Vec<_>>()
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    if let Ok((_, data)) = data {
        Some(
            data.iter()
                .map(|(s, v)| {
                    if get_all_operations(&v.iter().tuple_windows().collect_vec())
                        .filter_map(|x| x.iter().fold(None, operations_folder))
                        .any(|x| x == *s)
                    {
                        *s
                    } else {
                        0
                    }
                })
                .sum(),
        )
    } else {
        None
    }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
