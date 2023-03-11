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

fn is_ranges_overlapping(range1: &str, range2: &str) -> bool {
    let (min1, max1) = parse_range(range1);
    let (min2, max2) = parse_range(range2);

    (min1 <= min2 && min2 <= max1) || (min2 <= min1 && min1 <= max2)
}

fn part1(contents: &str) -> usize {
    let contained_ranges_count = contents
        .lines()
        .map(|line| {
            let pairs = line.split(',').collect::<Vec<&str>>();
            (pairs[0], pairs[1])
        })
        .filter(|&(range1, range2)| is_range_fully_contains(range1, range2))
        .count();

    contained_ranges_count
}

fn part2(contents: &str) -> usize {
    let overlapping_ranges_count = contents
        .lines()
        .map(|line| {
            let pairs = line.split(',').collect::<Vec<&str>>();
            (pairs[0], pairs[1])
        })
        .filter(|&(range1, range2)| is_ranges_overlapping(range1, range2))
        .count();

    overlapping_ranges_count
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1 answer = {}", part1(input));
    println!("Part 2 answer = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let range_not_1 = "2-4";
        let range_not_2 = "6-8";

        assert_eq!(is_range_fully_contains(range_not_1, range_not_2), false);
    }

    #[test]
    fn is_overlapping() {
        let range_overlap_1 = "5-7";
        let range_overlap_2 = "7-9";

        assert_eq!(
            is_ranges_overlapping(range_overlap_1, range_overlap_2),
            true
        );

        let range_not_1 = "2-4";
        let range_not_2 = "6-8";

        assert_eq!(is_ranges_overlapping(range_not_1, range_not_2), false);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input.txt");
        assert_eq!(super::part1(input), 534);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input.txt");
        assert_eq!(super::part2(input), 841);
    }
}
