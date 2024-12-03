use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::{map_res, value},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    FindSubstring, IResult,
};

advent_of_code::solution!(3);

fn parse_integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_args(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    separated_pair(parse_integer, char(','), parse_integer)(input).map(|(i, r)| (i, Some(r)))
}

fn parse_parens(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    delimited(tag("("), parse_args, tag(")"))(input)
}

fn parse_multiply(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    preceded(tag("mul"), parse_parens)(input)
}

fn parse_multiply_with_garbage(input: &str) -> IResult<&str, Option<(u32, u32)>> {
    alt((
        preceded(take_until("mul("), parse_multiply),
        value(None, preceded(take_until("mul("), tag("mul("))),
    ))(input)
}

fn parse_multiple(input: &str) -> IResult<&str, Vec<Option<(u32, u32)>>> {
    many0(parse_multiply_with_garbage)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, result) = parse_multiple(input)?;

    let result = result.into_iter().flatten().collect();

    Ok((input, result))
}

pub fn part_one(input: &str) -> Option<u32> {
    if let Ok((_remainder, data)) = parse(input) {
        Some(data.iter().fold(0, |acc, (a, b)| acc + a * b))
    } else {
        None
    }
}

fn remove_dont_do(input: &str) -> String {
    let dont_string = "don't()";
    let do_string = "do()";

    let mut builder = String::new();
    let mut valid_index = 0;
    let mut no_final_do = true;
    loop {
        let input = &input[valid_index..];
        let dont_index = input.find_substring(dont_string);
        if let Some(dont_index) = dont_index {
            builder.push_str(&input[..dont_index]);

            let input = &input[dont_index + dont_string.len()..];
            let do_index = input.find_substring(do_string);

            if let Some(do_index) = do_index {
                valid_index += dont_index + dont_string.len() + do_index + do_string.len();
            } else {
                no_final_do = false;
            }
        } else {
            if no_final_do {
                builder.push_str(input);
            }
            return builder;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let trimmed = remove_dont_do(input);

    part_one(&trimmed)
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
        assert_eq!(result, Some(48));
    }
}
