use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules = input
        .lines()
        .take_while(|line| line.len() > 1)
        .map(|line| {
            line.split("|")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|pages| (pages[0], pages[1]))
        .collect::<std::collections::HashSet<_>>();
    let result: usize = input
        .lines()
        .skip(rules.len() + 1)
        .map(|update| {
            update
                .split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|update| update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result as u32)

    // let (s1, s2) = input.split_once("\n\n").unwrap();
    // let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    // for l in s1.lines() {
    //     let (x, y) = l.split_once('|').unwrap();
    //     orderings.entry(y.parse().unwrap()).or_default().insert(x.parse().unwrap());
    // }
    // let pages = s2.lines().map(|l| {
    //     l.split(',').map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>()
    // });
    //
    // let (mut p1, mut p2) = (0, 0);
    // for mut p in pages {
    //     if p.is_sorted_by(|a, b| orderings[b].contains(a)) {
    //         p1 += p[p.len() / 2];
    //     } else {
    //         p.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
    //         p2 += p[p.len() / 2];
    //     }
    // }
    // // (p1, p2)
    //
    // return Some(p1 as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = input
        .lines()
        .take_while(|line| line.len() > 1)
        .map(|line| {
            line.split("|")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .map(|pages| (pages[0], pages[1]))
        .collect::<std::collections::HashSet<_>>();
    let result: usize = input
        .lines()
        .skip(rules.len() + 1)
        .map(|update| {
            update
                .split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|update| !update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))))
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum();

    Some(result as u32)

    // let (s1, s2) = input.split_once("\n\n").unwrap();
    // let mut orderings = HashMap::<usize, HashSet<usize>>::new();
    // for l in s1.lines() {
    //     let (x, y) = l.split_once('|').unwrap();
    //     orderings.entry(y.parse().unwrap()).or_default().insert(x.parse().unwrap());
    // }
    // let pages = s2.lines().map(|l| {
    //     l.split(',').map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>()
    // });
    //
    // let (mut p1, mut p2) = (0, 0);
    // for mut p in pages {
    //     if p.is_sorted_by(|a, b| orderings[b].contains(a)) {
    //         p1 += p[p.len() / 2];
    //     } else {
    //         p.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
    //         p2 += p[p.len() / 2];
    //     }
    // }
    // // (p1, p2)
    //
    // return Some(p2 as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
