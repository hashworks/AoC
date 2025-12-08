# Advent of Code 2025

https://adventofcode.com/2025/

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

|  Day |   Part 1 |   Part 2 | Notes |
| ---: | -------: | -------: | ----: |
|   01 | 00:27:44 | 01:01:44 |       |
|   02 | 00:41:42 | 02:00:37 |       |
|   03 | 00:13:35 | 01:09:11 |       |
|   04 | 00:19:00 | 00:54:10 |       |
|   05 |  unknown |  unknown |       |
|   06 |  unknown |  unknown |       |
|   07 |  unknown |  unknown |       |
|   08 |  unknown |  unknown |       |
|   09 |          |          |       |
|   10 |          |          |       |
|   11 |          |          |       |
|   12 |          |          |       |

## Performance

Please note that the following numbers are relative, and should only be used to compare the performance between commits and days.

|  Day |  Parsing |   Part 1 |   Part 2 | Notes |
| ---: | -------: | -------: | -------: | ----: |
|   01 | 333.72µs |  15.47µs | 443.90µs |       |
|   02 |  22.52µs |  30.45ms | 126.34ms |       |
|   03 | 165.15µs |  15.73µs |  39.98µs |       |
|   04 | 165.14µs | 209.85µs |  26.44ms |       |
|   05 | 152.63µs |  89.53µs |  24.40µs |       |
|   06 |   3.07ms |  94.17µs |  75.44µs |       |
|   07 |  86.26µs | 334.97µs | 312.42µs |       |
|   08 | 94.658µs |  30.06ms |  29.93ms |       |
|   09 |          |          |          |       |
|   10 |          |          |          |       |
|   11 |          |          |          |       |
|   12 |          |          |          |       |
