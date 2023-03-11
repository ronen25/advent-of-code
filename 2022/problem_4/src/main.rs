fn parse_range(range: &str) -> (i32, i32) {
    let parts: Vec<i32> = range
        .split('-')
        .map(|part| part.parse::<i32>().unwrap())
        .collect();

    (parts[0], parts[1])
}

fn is_range_fully_contains(range1: &str, range2: &str) -> bool {
    let (min1, max1) = parse_range(range1);
    let (min2, max2) = parse_range(range2);

    // One of the ranges may contain each other; meaning, the order doesn't matter.
    (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
}

fn part1(contents: &str) {
    let mut count = 0;

    for line in contents.lines() {
        let pairs = line.split(',').collect::<Vec<&str>>();
        let (range1, range2) = (pairs[0], pairs[1]);

        let is_contained = is_range_fully_contains(range1, range2);
        if is_contained {
            count += 1;
        }
    }

    println!("{}", count);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn parse_range() {
        let range_example = "90-34";
        let (min, max) = super::parse_range(range_example);

        assert_eq!(min, 90);
        assert_eq!(max, 34);
    }

    #[test]
    fn in_range() {
        let range1 = "3-7";
        let range2 = "2-8";

        assert_eq!(is_range_fully_contains(range1, range2), true);
    }

    #[test]
    fn not_in_range() {
        let range1 = "2-4";
        let range2 = "6-8";

        assert_eq!(is_range_fully_contains(range1, range2), false);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        println!("{}", input);
    }
}
