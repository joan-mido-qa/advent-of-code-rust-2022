pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split(&[',', '-'])
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|s| {
                if (s[0] <= s[2] && s[1] >= s[3]) || (s[2] <= s[0] && s[3] >= s[1]) {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split(&[',', '-'])
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|s| {
                if (s[0] <= s[3] && s[1] >= s[2]) || (s[3] <= s[0] && s[2] >= s[1]) {
                    return 1;
                };
                0
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
