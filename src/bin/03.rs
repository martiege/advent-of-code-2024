use nom::{bytes::complete::tag, character::complete::{char, digit1}, combinator::map_res, multi::many0, sequence::{preceded, separated_pair, tuple}, IResult};

advent_of_code::solution!(3);

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_args(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        parse_integer,
        char(','),
        parse_integer,
    )(input)
}

fn parse_parens(input: &str) -> IResult<&str, (u32, u32)> {
    tuple((
        tag("("),
        parse_args,
        tag(")"),
    ))(input).map(|(i, (_, x, _))| (i, x))
}

fn parse_multiply(input: &str) -> IResult<&str, (u32, u32)> {
    preceded(
        tag("mul"),
        parse_parens,
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many0(parse_multiply)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    dbg!(parse(input));

    None
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
