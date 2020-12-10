# Example Code

In this repo you will find my optimization steps I undertook as part of Advent
Of Code 2020 (day 9).

Originally, I did not create a separate function for every modification I tried.
This is because, _Criterion_ provides a comparison against the last run by
default. Also some of the optimizations are pretty obvious (keep in mind that I
wrote the original solution against the clock.).

I am not including the file parsing in the benchmarks. I should really include
the file parsing as a separate benchmark but I wasn't so interested in this, so
far.

Please create a PR if you can come up with a faster implementation. Or with an
implementation that is similarly fast but interesting in some way. (For
practical reasons, I'll use my personal machine to measure performance.)

Reference: https://twitter.com/chsiedentop/status/1336598864267104258?s=20

## Example Output

Call with `cargo bench`

```shell
Part 1/impl A           time:   [2.4445 ms 2.4675 ms 2.4928 ms]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Part 1/impl B           time:   [2.4993 ms 2.5888 ms 2.7104 ms]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
Part 1/impl C           time:   [174.27 us 177.48 us 180.98 us]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
Part 1/impl D           time:   [65.932 us 66.504 us 67.059 us]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Part 1/impl E           time:   [65.716 us 66.430 us 67.333 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Part 2/impl A           time:   [141.65 us 143.10 us 144.45 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Part 2/impl B           time:   [11.896 us 12.024 us 12.152 us]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```
