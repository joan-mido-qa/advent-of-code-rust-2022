use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    for i in 4..input.len() {
        let mut uniq = HashSet::new();

        if input[(i - 4)..i].chars().all(move |x| uniq.insert(x)) {
            return Some(i);
        };
    }

    None
}

pub fn part_two(input: &str) -> Option<usize> {
    for i in 14..input.len() {
        let mut uniq = HashSet::new();

        if input[(i - 14)..i].chars().all(move |x| uniq.insert(x)) {
            return Some(i);
        };
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
