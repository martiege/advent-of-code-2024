use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

fn parse(input: &str) -> IResult<&str, Vec<(usize, Vec<u32>)>> {
    separated_list1(
        tag("\n"),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<usize>()),
            tag(": "),
            separated_list0(tag(" "), map_res(digit1, |s: &str| s.parse::<u32>())),
        ),
    )(input)
}

fn matches(sum: usize, vector: &[u32], operators: &[fn(usize, usize) -> usize]) -> bool {
    let mut results = HashSet::new();
    results.insert(0);
    results.insert(1);

    for v in vector {
        let mut new_results = HashSet::new();
        for r in results {
            if r > sum {
                continue;
            }
            for o in operators {
                new_results.insert(
                    o(r, *v as usize)
                );
            }
            // let t = r + *v as usize;
            // new_results.insert(t);
            // let t = r * *v as usize;
            // new_results.insert(t);
        }
        results = new_results;
    }

    results.contains(&sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = parse(input);

    if let Ok((rest, data)) = data {
        assert!(rest.trim().is_empty());
        Some(
            data.par_iter()
                .map(|(s, v)| if matches(*s, v, &[|a, b| a + b, |a, b| a * b]) { *s } else { 0 })
                .sum(),
        )
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = parse(input);
    let operators = [
        |a, b| a + b,
        |a, b| a * b,
        |a: usize, b: usize| (a.to_string() + &b.to_string()).parse::<usize>().expect("this shouldn't fail")
    ];

    if let Ok((rest, data)) = data {
        assert!(rest.trim().is_empty());
        Some(
            data.par_iter()
                .map(|(s, v)| if matches(*s, v, &operators) { *s } else { 0 })
                .sum(),
        )
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
