use std::{collections::HashMap, mem};

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

macro_rules! update_or_create {
    ($map:expr, $key:expr, $val:expr) => {{
        $map.entry($key)
            .and_modify(|x| {
                *x += $val;
            })
            .or_insert($val);
    }};
}

fn count(data: Vec<usize>, n: usize) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for x in data {
        update_or_create!(counts, x, 1);
    }

    for _ in 0..n {
        let mut new: HashMap<usize, usize> = HashMap::new();

        for (&k, &v) in counts.iter() {
            if k == 0 {
                update_or_create!(new, 1, v);
            } else {
                let digits = k.ilog10() + 1;

                if digits % 2 == 0 {
                    let split = digits / 2;
                    let pow = (10usize).pow(split);
                    let left = k / pow;
                    let right = k - left * pow;

                    update_or_create!(new, left, v);
                    update_or_create!(new, right, v);
                } else {
                    update_or_create!(new, k * 2024, v);
                }
            }
        }

        // counts = new;
        mem::swap(&mut counts, &mut new);
    }

    counts.values().sum()
}

fn get_stone_count(data: &Vec<usize>, blinks: usize) -> usize {
    // numbers->counts
    let mut current: HashMap<usize, usize> = HashMap::new();

    for &datum in data {
        update_or_create!(current, datum, 1);
    }

    // take x blinks into eternity
    for _ in (0..).take(blinks) {
        // had to make sure these were the same types
        let mut next: HashMap<usize, usize> = HashMap::new();

        for (&k, &v) in current.iter() {
            if k == 0 {
                // pass the same number of items into the "1" key
                update_or_create!(next, 1, v);
                continue;
            }

            // get number of digits in the number
            let digits = k.ilog10() + 1;

            if digits % 2 == 0 {
                // split into 2
                let split = digits / 2;
                let pow = (10usize).pow(split);
                let left = k / pow;
                let right = k - left * pow;

                update_or_create!(next, left, v);
                update_or_create!(next, right, v);
            } else {
                // just multiply by 2024
                update_or_create!(next, k * 2024, v);
            }
        }

        // first mem::swap
        mem::swap(&mut current, &mut next);
    }

    current.values().sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    if let Ok((_, data)) = parse(input) {
        Some(count(data, 25))
        // Some(get_stone_count(&data, 25))
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    if let Ok((_, data)) = parse(input) {
        Some(count(data, 75))
        // Some(get_stone_count(&data, 75))
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
