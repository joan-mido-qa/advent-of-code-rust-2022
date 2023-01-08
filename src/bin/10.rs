pub fn part_one(input: &str) -> Option<i32> {
    let signals = vec![20, 60, 100, 140, 180, 220];

    let mut count = 0;
    let mut cycle = 1;
    let mut value = 1;

    input.lines().for_each(|line| {
        match line {
            "noop" => cycle += 1,
            _ => {
                cycle += 1;

                if signals.contains(&cycle) {
                    count += cycle * value
                }

                cycle += 1;
                value += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            }
        };

        if signals.contains(&cycle) {
            count += cycle * value
        }
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut current: String = String::new();

    let mut crt = String::new();

    let mut cycle = 1;
    let mut value = 1;

    input.lines().for_each(|line| {
        let pos: i32 = (current.len() + 1) as i32;

        match (value..(value + 3)).contains(&pos) {
            true => current.push('#'),
            false => current.push('.'),
        };

        if current.len() % 40 == 0 {
            crt = format!("{crt}\n{current}");

            current.clear()
        }

        match line {
            "noop" => cycle += 1,
            _ => {
                cycle += 1;

                let pos: i32 = (current.len() + 1) as i32;

                match (value..(value + 3)).contains(&pos) {
                    true => current.push('#'),
                    false => current.push('.'),
                };

                if current.len() % 40 == 0 {
                    crt = format!("{crt}\n{current}");

                    current.clear()
                }

                cycle += 1;
                value += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            }
        };
    });

    Some(crt.trim().to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....".to_string()));
    }
}
