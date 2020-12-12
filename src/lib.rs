use eyre::Result;
use eyre::{eyre, Context};
use itertools::Itertools;
use std::{convert::TryInto, str::FromStr};

/// Originally, I used find_sum() function. Even with inline that was slower.
/// So, instead I included the check inside the loop.
pub fn part1(numbers: &[u64], preamble: usize) -> Option<u64> {
    for window in numbers.windows(preamble + 1) {
        let target = window[preamble];

        let has_sum = window
            .iter()
            .combinations(2)
            .any(|p| *p[0] + *p[1] == target);
        if !has_sum {
            return Some(target);
        }
    }
    None
}

/// Virtually the same as impl A.
pub fn part1_b(numbers: &[u64], preamble: usize) -> Option<u64> {
    for window in numbers.windows(preamble + 1) {
        let target = window[preamble];

        let has_sum = (0..preamble)
            .map(|j| ((j + 1)..preamble).map(move |k| (j, k)))
            .flatten()
            .any(|(j, k)| window[j] + window[k] == target);

        if !has_sum {
            return Some(target);
        }
    }
    None
}

const PREAMBLE: usize = 25;
pub fn part1_f(a: &Vec<u64>, _preamble: usize) -> Option<u64> {
    for i in PREAMBLE..a.len() {
        let window: [u64; PREAMBLE] = a[i - PREAMBLE..i].try_into().unwrap();
        // let window = a[i - PREAMBLE..i].to_vec();
        // window.sort_unstable();
        if !has_sum2(&window, a[i]) {
            return Some(a[i]);
        }
    }

    None
}
// Same as above has_sum but using a linear search for hopefully better cache perf on small slices
#[inline]
fn has_sum2(a: &[u64; PREAMBLE], target: u64) -> bool {
    // fn has_sum2(a: &[u64], target: u64) -> bool {
    for i in 0..a.len() {
        for j in (i + 1)..a.len() {
            if a[i] + a[j] == target {
                return true;
            }
        }
    }
    false
}

/// Pre-compute the sliding window sums for each combination in the window.
/// This has better complexity (much faster!).
/// Downside: Memory overhead.
/// Each inner vector is of length (preamble - 1). Total size: (N-K) * (K-1)= O(N*K) if K << N.
///
/// Runtime is (N-K) * (K-1) [build the sums data] + (N-K) * (K) [iterate through the sums]
/// = O(N*K)
///
/// K := preamble
/// 160us
pub fn part1_c(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    // const preamble: usize = 25;

    let mut sums = Vec::new();
    for i in 0..(numbers.len() - preamble) {
        sums.push(
            (1..preamble)
                .map(|j| numbers[i] + numbers[i + j])
                .collect::<Vec<_>>(),
        );
    }

    let sums = sums;

    for i in 0..(numbers.len() - preamble) {
        let target = numbers[i + preamble];

        let has_sum = sums[i..(i + preamble)]
            .iter()
            .find(|v| v.iter().any(|p| *p == target))
            .is_some();
        if !has_sum {
            return Some(target);
        }
    }
    None
}

/// Same as E. But instead of using a Vec<Vec<_>> it is just a one dimensional
/// array Vec<_>. This is obviously much better for memory access.
/// 60us
pub fn part1_d(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    let mut sums = Vec::new();
    for i in 0..(numbers.len() - preamble) {
        for j in 1..preamble {
            sums.push(numbers[i] + numbers[i + j]);
        }
    }

    let sums = sums;

    let window_size = preamble * (preamble - 1);
    for i in 0..(numbers.len() - preamble) {
        let target = numbers[i + preamble];
        let start = (preamble - 1) * i;
        let end = start + window_size;

        let has_sum = sums[start..end].iter().any(|p| *p == target);
        if !has_sum {
            return Some(target);
        }
    }
    None
}

/// Same as D. But functional-style to find the bad value.
pub fn part1_e(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    let mut sums = Vec::new();
    for i in 0..(numbers.len() - preamble) {
        for j in 1..preamble {
            sums.push(numbers[i] + numbers[i + j]);
        }
    }
    let sums = sums;

    // NOTE: This is 50% slower than the hand written version!
    // let sums: Vec<_> = (0..(numbers.len() - preamble))
    //     .flat_map(|i| (1..preamble).map(move |j| numbers[i] + numbers[i + j]))
    //     .collect();

    let window_size = preamble * (preamble - 1);
    (0..(numbers.len() - preamble)).find_map(|i| {
        let target = numbers[i + preamble];
        let start = (preamble - 1) * i;
        let end = start + window_size;

        let has_sum = sums[start..end].iter().any(|p| *p == target);
        if !has_sum {
            Some(target)
        } else {
            None
        }
    })
}

pub mod believer {
    /// NOTE: License: All code in this section is owned by https://github.com/believer/advent-of-code
    /// The repo does not have a License file attached, which means that all Copyright belongs to @believer (Rickard Natt och Dag).
    use itertools::iproduct;
    use std::collections::HashSet;

    /// https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_09.rs
    fn calculate_sums(input: &[u64]) -> Vec<u64> {
        iproduct!(input.iter(), input.iter())
            .map(|tuple| tuple.0 + tuple.1)
            .collect()
    }

    /// https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_09.rs
    pub fn find_broken_number(input: &[u64], preamble: usize) -> u64 {
        let mut not_in: HashSet<u64> = HashSet::new();

        for i in preamble..input.len() {
            let (previous, _) = input.split_at(i);
            let (_, last_five) = previous.split_at(i - preamble);
            let sums = calculate_sums(last_five);

            let v = input[i];

            if !sums.contains(&v) {
                not_in.insert(v);
            }
        }

        *not_in.iter().next().unwrap()
    }

    /// Use slices directly
    pub fn find_broken_number_b(input: &[u64], preamble: usize) -> u64 {
        // let mut not_in: HashSet<u64> = HashSet::new();

        for i in preamble..input.len() {
            let last_five = &input[(i - preamble)..i];
            // let (previous, _) = input.split_at(i);
            // let (_, last_five) = previous.split_at(i - preamble);
            let sums = calculate_sums(last_five);

            let v = input[i];

            if !sums.contains(&v) {
                // not_in.insert(v);
                return v;
            }
        }

        unreachable!()
        // *not_in.iter().next().unwrap()
    }
}

pub mod benfrankel {
    /// NOTE: License: All code in this section is owned by https://github.com/benfrankel/aoc.
    /// The repo does not have a License file attached, which means that all Copyright belongs to Ben Frankel.
    use std::cmp::Ordering;

    /// https://github.com/benfrankel/aoc/blob/2b6797cea62f95f8c80eae45ecbc76b973fab239/src/util.rs
    /// @benfrankel
    pub fn find_sum2(a: &[i64], target: i64) -> Option<(usize, usize)> {
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            let sum = a[i] + a[j];
            match sum.cmp(&target) {
                Ordering::Equal => return Some((i, j)),
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
        }

        None
    }

    /// https://github.com/benfrankel/aoc/blob/e7ea2cbc3814765ede9bbcd094be65b916c51b44/src/aoc/year2020/day09.rs#L18
    /// @benfrankel
    pub fn part1(a: &[i64], preamble: usize) -> i64 {
        // let preamble = 25;  Converted to argument by @siedentop to make it comparable to other functions.

        for i in preamble..a.len() {
            let mut window = a[i - preamble..i].to_vec();
            window.sort_unstable();
            if find_sum2(&window, a[i]).is_none() {
                return a[i];
            }
        }

        panic!("Couldn't find an appropriate window.")
    }

    /// Use &[u64] to make it comparable to the other implementations.
    /// by @siedentop.
    pub fn part1_b(a: &[u64], preamble: usize) -> u64 {
        for i in preamble..a.len() {
            let mut window = a[i - preamble..i].to_vec();
            window.sort_unstable();
            if find_sum2_b(&window, a[i]).is_none() {
                return a[i];
            }
        }

        panic!("Couldn't find an appropriate window.")
    }
    // Same as above, but taking u64 not i64.
    fn find_sum2_b(a: &[u64], target: u64) -> Option<(usize, usize)> {
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            let sum = a[i] + a[j];
            match sum.cmp(&target) {
                Ordering::Equal => return Some((i, j)),
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
        }

        None
    }

    /// Use &[u64] to make it comparable to the other implementations.
    /// by @siedentop.
    pub fn part1_c(a: &[u64], preamble: usize) -> u64 {
        for i in preamble..a.len() {
            let mut window = a[i - preamble..i].to_vec();
            window.sort_unstable();
            if !has_sum(&window, a[i]) {
                return a[i];
            }
        }

        panic!("Couldn't find an appropriate window.")
    }
    // Same as above find_sum2 but returning only true/false.
    #[inline]
    fn has_sum(a: &[u64], target: u64) -> bool {
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            let sum = a[i] + a[j];
            match sum.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
        }
        false
    }

    /// Use &[u64] to make it comparable to the other implementations.
    /// by @siedentop.
    pub fn part1_d(a: &[u64], preamble: usize) -> u64 {
        for i in preamble..a.len() {
            let mut window = a[i - preamble..i].to_vec();
            window.sort_unstable();
            if !has_sum2(&window, a[i]) {
                return a[i];
            }
        }

        panic!("Couldn't find an appropriate window.")
    }
    // Same as above has_sum but using a linear search for hopefully better cache perf on small slices
    #[inline]
    fn has_sum2(a: &[u64], target: u64) -> bool {
        for i in 0..a.len() {
            for j in (i + 1)..a.len() {
                if a[i] + a[j] == target {
                    return true;
                }
            }
        }
        false
    }
}

/// Naive implementation. But originally faster than part 1!
/// O(n^2)
/// 180us
pub fn part2(numbers: &Vec<u64>, target: u64) -> Result<u64> {
    let running_sum: Vec<_> = numbers
        .iter()
        .scan(0u64, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .collect();

    for start in 0..running_sum.len() {
        // a bit of binary search here would be nice...
        for end in start..running_sum.len() {
            let window_sum = running_sum[end] - running_sum[start];
            if window_sum == target {
                let lowest = numbers[start..end].iter().min().unwrap();
                let highest = numbers[start..end].iter().max().unwrap();

                return Ok(lowest + highest);
            }
        }
    }

    Err(eyre!("no result"))
}

/// Implementation uses binary search to find the end index. O(n * log(n)).
/// 12us on my benchmark.
pub fn part2_b(numbers: &Vec<u64>, target: u64) -> Result<u64> {
    let numbers = numbers.clone();

    let running_sum: Vec<_> = numbers
        .iter()
        .scan(0u64, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .collect();

    for start in 0..running_sum.len() {
        let needed = running_sum[start] + target;
        let end = running_sum[start..].binary_search(&needed);

        if let Ok(end) = end {
            let end = start + end;
            let lowest = numbers[start..end].iter().min().unwrap();
            let highest = numbers[start..end].iter().max().unwrap();

            return Ok(lowest + highest);
        }
    }

    Err(eyre!("no result"))
}

#[inline]
#[allow(unused)]
fn find_sum(window: &[u64]) -> bool {
    let preamble = window.len() - 1;
    let target = window[preamble];

    let nums = window.iter().take(preamble);

    let res = nums.combinations(2).find(|p| *p[0] + *p[1] == target);
    res.is_some()
}

pub fn parse_file<'a, I>(rows: I) -> Result<Vec<u64>>
where
    I: IntoIterator<Item = String>,
{
    rows.into_iter().map(|r| parse_row(&r)).collect()
}

pub fn parse_row(row: &str) -> Result<u64> {
    u64::from_str(row).wrap_err(format!("row failed: {}", row))
}
