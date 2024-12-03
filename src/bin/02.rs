use std::cmp::max;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

advent_of_code::solution!(2);

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(" "), parse_integer)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn are_all_elements_equal<T: PartialEq>(elems: &[T]) -> bool {
    match elems {
        [head, tail @ ..] => tail.iter().all(|x| x == head),
        [] => false,
    }
}

fn accumulate_safe(acc: usize, data: &[u32]) -> usize {
    let diffs = data
        .windows(2)
        .map(|p| (p[0] as i64 - p[1] as i64))
        .collect::<Vec<_>>();

    let signs = diffs.iter().map(|x| x.signum()).collect::<Vec<_>>();

    if !are_all_elements_equal(&signs) {
        // if not strictly increasing or decreasing
        acc
    } else if !diffs.iter().all(|x| (1..=3).contains(&x.abs())) {
        // if not strictly increasing or decreasing with [1, 3] levels
        acc
    } else {
        acc + 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((_, data)) = parse(input) {
        Some(data.iter().fold(0, |arg0: usize, arg1: &Vec<u32>| {
            accumulate_safe(arg0, arg1)
        }) as u32)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let Ok((_, data)) = parse(input) {
        Some(data.iter().fold(0, |acc, x| {
            max(
                (0..x.len()).fold(0, |innacc, i| {
                    let mut subdata = x.clone();
                    subdata.remove(i);
                    max(innacc, accumulate_safe(0, &subdata))
                }),
                accumulate_safe(0, x),
            ) + acc
        }) as u32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
