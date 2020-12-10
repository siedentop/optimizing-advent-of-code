use eyre::Result;
use eyre::{eyre, Context};
use itertools::Itertools;
use std::str::FromStr;

/// Originally, I used find_sum() function. Even with inline that was slower.
/// So, instead I included the check inside the loop.
pub fn part1(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    for window in numbers.windows(preamble + 1) {
        let target = window[preamble];

        let res = window
            .iter()
            .combinations(2)
            .find(|p| *p[0] + *p[1] == target);
        if res.is_none() {
            return Some(*window.last().unwrap());
        }
    }
    None
}

/// Virtually the same as impl A.
pub fn part1_b(numbers: &Vec<u64>, preamble: usize) -> Option<u64> {
    numbers.windows(preamble + 1).find_map(|window| {
        let target = window[preamble];

        if window
            .iter()
            .combinations(2)
            .find(|p| *p[0] + *p[1] == target)
            .is_none()
        {
            Some(target)
        } else {
            None
        }
    })
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
