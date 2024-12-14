# Advent of Code 2024

https://adventofcode.com/2024/

Solutions will be done in Rust.

## Running

```bash
cargo run --release --bin day01
```

## Testing

```bash
cargo test --bin day01
```

## ENV Variables

`INPUT` can be set to provide a path to a specific file as input. Otherwise, `inputs/day{ID}.txt` will be used.

## Personal Times

Times are normalized from input download to correct answer, not the time on the website.

|  Day |   Part 1 | Rank Part 1 |   Part 2 | Rank Part 2 | Notes |
| ---: | -------: | ----------: | -------: | ----------: | ----: |
|   01 | 00:12:00 |       28688 | 00:17:00 |       26554 |       |
|   02 | 00:12:34 |        5392 | 00:30:03 |        5166 |       |
|   03 | 00:14:49 |        7450 | 01:00:21 |       11097 |       |
|   04 | 00:45:31 |       26392 | 01:12:24 |       22626 |       |
|   05 | 00:53:47 |       10330 | 01:04:59 |        7581 |       |
|   06 |    later |       25244 |    later |       53516 |       |
|   07 | 00:36:51 |        8267 |    later |       16579 |       |
|   08 | 00:34:58 |        9283 | 00:45:04 |        8481 |       |
|   09 | 00:37:26 |        8754 | 01:33:13 |        6521 |       |
|   10 | 00:33:11 |       17323 | 00:34:53 |       16306 |       |
|   11 | 00:21:18 |       21968 | 03:32:15 |       19531 |       |
|   12 | 00:44:31 |       20042 |    later |       14517 |       |
|   13 |    later |       28728 |    later |       23186 |       |
|   14 | 00:52:59 |       18356 | 01:24:56 |       14648 |       |
|   15 |          |             |          |             |       |
|   16 |          |             |          |             |       |
|   17 |          |             |          |             |       |
|   18 |          |             |          |             |       |
|   19 |          |             |          |             |       |
|   20 |          |             |          |             |       |
|   21 |          |             |          |             |       |
|   22 |          |             |          |             |       |
|   23 |          |             |          |             |       |
|   24 |          |             |          |             |       |
|   25 |          |             |          |             |       |

## Performance

Please note that the following numbers are relative, and should only be used to compare the performance between commits and days.

|  Day |   Parsing |    Part 1 |    Part 2 |       Notes |
| ---: | --------: | --------: | --------: | ----------: |
|   01 | 119.845µs |  39.323µs |  27.732µs |             |
|   02 | 250.591µs |  14.197µs | 237.767µs |             |
|   03 |  31.839µs | 447.642µs | 790.296µs |             |
|   04 |  70.973µs | 354.837µs | 107.843µs |             |
|   05 | 217.840µs |  23.644µs | 100.199µs |             |
|   09 |  92.665µs | 674.419µs | 119.509ms | brute force |
|   07 | 339.549µs | 472.799µs |  18.917ms |             |
|   08 |  41.918µs |  52.208µs | 168.617µs |             |
|   09 | 885.015µs | 939.037µs |  21.743ms |             |
|   10 |  42.760µs | 346.121µs | 174.288µs |             |
|   11 |  12.493µs | 432.634µs |  13.627ms |             |
|   12 |  71.304µs |   3.011ms |   2.718ms |             |
|   13 | 123.994µs |   2.926µs |   2.525µs |             |
|   14 |  64.952µs |   6.722µs |  16.433ms |             |
|   15 |           |           |           |             |
|   16 |           |           |           |             |
|   17 |           |           |           |             |
|   18 |           |           |           |             |
|   19 |           |           |           |             |
|   20 |           |           |           |             |
|   21 |           |           |           |             |
|   22 |           |           |           |             |
|   23 |           |           |           |             |
|   24 |           |           |           |             |
|   25 |           |           |           |             |
