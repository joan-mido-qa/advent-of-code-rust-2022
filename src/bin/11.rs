#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    test: u64,
    pass: usize,
    fail: usize,
    count: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let m = monkey.lines().collect::<Vec<&str>>();

            Monkey {
                items: m[1]
                    .trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(",")
                    .map(|item| item.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
                operation: m[2]
                    .trim()
                    .strip_prefix("Operation: new = ")
                    .unwrap()
                    .to_string(),
                test: m[3]
                    .trim()
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                pass: m[4]
                    .trim()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                fail: m[5]
                    .trim()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                count: 0,
            }
        })
        .collect::<Vec<Monkey>>();

    let mut items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
    let mo: u64 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..20 {
        for (n, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut items[n]);

            for item in monkey.items.drain(0..) {
                monkey.count += 1;

                let operation = monkey.operation.replace("old", &item.to_string());

                let op = operation.trim().split(" ").collect::<Vec<&str>>();

                let new = match op[1] {
                    "+" => (op[0].parse::<u64>().unwrap() + op[2].parse::<u64>().unwrap()) / 3 % mo,
                    "*" => (op[0].parse::<u64>().unwrap() * op[2].parse::<u64>().unwrap()) / 3 % mo,
                    _ => 0,
                };

                if new % monkey.test == 0 {
                    items[monkey.pass].push(new);
                } else {
                    items[monkey.fail].push(new);
                }
            }
        }
    }

    let mut activity: Vec<u64> = monkeys.iter().map(|monkey| monkey.count).collect();

    activity.sort();

    Some(activity.iter().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let m = monkey.lines().collect::<Vec<&str>>();

            Monkey {
                items: m[1]
                    .trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(",")
                    .map(|item| item.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
                operation: m[2]
                    .trim()
                    .strip_prefix("Operation: new = ")
                    .unwrap()
                    .to_string(),
                test: m[3]
                    .trim()
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                pass: m[4]
                    .trim()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                fail: m[5]
                    .trim()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                count: 0,
            }
        })
        .collect::<Vec<Monkey>>();

    let mut items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
    let mo: u64 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..10000 {
        for (n, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut items[n]);

            for item in monkey.items.drain(0..) {
                monkey.count += 1;

                let operation = monkey.operation.replace("old", &item.to_string());

                let op = operation.trim().split(" ").collect::<Vec<&str>>();

                let new = match op[1] {
                    "+" => (op[0].parse::<u64>().unwrap() + op[2].parse::<u64>().unwrap()) % mo,
                    "*" => (op[0].parse::<u64>().unwrap() * op[2].parse::<u64>().unwrap()) % mo,
                    _ => 0,
                };

                if new % monkey.test == 0 {
                    items[monkey.pass].push(new);
                } else {
                    items[monkey.fail].push(new);
                }
            }
        }
    }

    let mut activity: Vec<u64> = monkeys.iter().map(|monkey| monkey.count).collect();

    activity.sort();

    Some(activity.iter().rev().take(2).product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
