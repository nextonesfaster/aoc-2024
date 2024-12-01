//! Problem URL: https://adventofcode.com/2024/day/1

use std::collections::HashMap;

type Lists = [Vec<u32>; 2];

#[aoc_generator(day1)]
fn get_lists(input: &str) -> Lists {
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for line in input.lines() {
        let mut splits = line.split_ascii_whitespace();
        list_one.push(splits.next().and_then(|s| s.parse().ok()).expect("expected proper value 1"));
        list_two.push(splits.next().and_then(|s| s.parse().ok()).expect("expected proper value 2"));
    }

    [list_one, list_two]
}

#[aoc(day1, part1)]
pub fn part_one(lists: &Lists) -> u32 {
    let mut l1 = lists[0].clone();
    l1.sort_unstable();
    let mut l2 = lists[1].clone();
    l2.sort_unstable();
    l1.iter().zip(l2).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2, iter_map)]
pub fn part_two_iter_map(lists: &Lists) -> u32 {
    let mut counts_map = HashMap::new();
    for (&v1, &v2) in lists[0].iter().zip(&lists[1]) {
        counts_map.entry(v1).or_insert((0, 0)).0 += 1;
        counts_map.entry(v2).or_insert((0, 0)).1 += 1;
    }
    counts_map.into_iter().fold(0, |acc, (v, (c1, c2))| acc + v * c1 * c2)
}

#[aoc(day1, part2, iter_l1)]
pub fn part_two_iter_l1(lists: &Lists) -> u32 {
    let mut counts_map = HashMap::new();
    for &v in &lists[1] {
        *counts_map.entry(v).or_insert(0) += 1;
    }
    lists[0].iter().filter_map(|v| counts_map.get(v).map(|c| c * v)).sum()
}

#[cfg(test)]
mod test_day_1 {
    use super::Lists;
    use crate::problem::day_01::{get_lists, part_one, part_two_iter_l1, part_two_iter_map};

    fn get_input() -> Lists {
        get_lists("3   4\n4   3\n2   5\n1   3\n3   9\n3   3")
    }

    #[test]
    fn test_get_lists() {
        assert_eq!([vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]], get_input());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(11, part_one(&get_input()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(31, part_two_iter_map(&get_input()));
        assert_eq!(31, part_two_iter_l1(&get_input()));
    }
}
