advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    File,
    Free,
}

#[derive(Debug)]
struct Block {
    type_: BlockType,
    data: Vec<usize>,
    free: usize,
}

impl Block {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Block {
                type_: BlockType::File,
                free: 0,
                data: vec![id / 2; size],
            }
        } else {
            Block {
                type_: BlockType::Free,
                free: size,
                data: Vec::new(),
            }
        }
    }

    fn fill(&mut self, data: &Vec<usize>) {
        self.data.extend(data);
        self.free -= data.len();
        if self.free == 0 {
            self.type_ = BlockType::File;
        }
    }

    fn clear(&mut self) {
        self.free = self.data.len();
        self.data.clear();
        self.type_ = BlockType::Free;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk: Vec<Option<usize>> = input
        .chars()
        .filter(|&c| c.is_ascii_digit()) // No whitespace.
        .filter_map(|c| c.to_digit(10)) // Convert to digit.
        .map(|c| c as usize) // We'll use this as the size of the vec.
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c]
            } else {
                vec![None; c]
            }
        })
        .collect();

    // Now that we have the disk, we should keep two pointers, one for the
    // left-most free space and one for the right most filled space.
    // We'll then swap and move to the next until they reach each other.
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        // Find the next free space.
        while left < right && disk[left].is_some() {
            left += 1;
        }

        // Find the last filled space.
        while left < right && disk[right].is_none() {
            right -= 1;
        }

        // Swap the two.
        disk.swap(left, right);

        // Increment the pointers.
        left += 1;
        right -= 1;
    }

    Some(
        disk.iter()
            .enumerate()
            .filter_map(|(i, &x)| x.map(|x| x * i))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk: Vec<Block> = input
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .enumerate()
        .map(|(i, size)| Block::new(i, size))
        .collect();

    let mut right = disk.len() - 1;
    while right > 0 {
        if disk[right].type_ == BlockType::Free {
            right -= 1;
            continue;
        }

        let mut left = 0;
        while left < right {
            if disk[left].type_ == BlockType::File
                || (disk[left].type_ == BlockType::Free && disk[left].free < disk[right].data.len())
            {
                left += 1;
                continue;
            }

            let r = disk[right].data.clone();
            disk[left].fill(&r);
            disk[right].clear();
            break;
        }
        right -= 1;
    }

    Some(
        disk.iter()
            .flat_map(|d| {
                d.data
                    .iter()
                    .cloned()
                    .chain(std::iter::repeat(0).take(d.free))
            })
            .enumerate()
            .map(|(i, x)| x * i)
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
