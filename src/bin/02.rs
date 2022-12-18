pub fn part_one(input: &str) -> Option<u32> {
    return Some(
        input
            .trim()
            .lines()
            .map(|line| match line.trim() {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => 0,
            })
            .sum(),
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(
        input
            .trim()
            .lines()
            .map(|line| match line.trim() {
                "A X" => 0 + 3,
                "A Y" => 3 + 1,
                "A Z" => 6 + 2,
                "B X" => 0 + 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 0 + 2,
                "C Y" => 3 + 3,
                "C Z" => 6 + 1,
                _ => 0,
            })
            .sum(),
    );
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
