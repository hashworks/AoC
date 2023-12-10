# Advent of Code 2023

https://adventofcode.com/2023/

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

|  Day |   Part 1 |   Part 2 | Notes |
| ---: | -------: | -------: | ----: |
|   01 | 00:18:44 | 00:49:15 |       |
|   02 | 00:24:47 | 00:28:03 |       |
|   03 | 01:28:32 | 01:50:54 |       |
|   04 | 00:18:15 | 00:57:59 |       |
|   05 | 01:14:30 | 01:33:45 |       |
|   06 | 00:35:49 | 00:56:21 |       |
|   07 | 01:14:12 | 02:33:41 |       |
|   08 | 00:34:04 | 02:50:34 |       |
|   09 | 00:37:47 | 00:45:58 |       |
|   10 | 02:35:19 | 07:45:00 |       |
|   11 |          |          |       |
|   12 |          |          |       |
|   13 |          |          |       |
|   14 |          |          |       |
|   15 |          |          |       |
|   16 |          |          |       |
|   17 |          |          |       |
|   18 |          |          |       |
|   19 |          |          |       |
|   20 |          |          |       |
|   21 |          |          |       |
|   22 |          |          |       |
|   23 |          |          |       |
|   24 |          |          |       |
|   25 |          |          |       |

## Performance

Please note that the following numbers are relative, and should only be used to compare the performance between commits and days.

|  Day |  Parsing |    Part 1 |   Part 2 |       Notes |
| ---: | -------: | --------: | -------: | ----------: |
|   01 | 196.99µs |   25.67µs | 124.33µs |             |
|   02 | 139.44µs |    1.07µs |    710ns |             |
|   03 |  70.04µs |   49.98µs |  64.74µs |             |
|   04 | 255.87µs |    1.20µs |   1.75µs |             |
|   05 |  36.29µs | 3.25.00µs |    9.13s | Brute Force |
|   06 |  12.36µs |  120.00ns |   1.71ms |             |
|   07 | 303.80µs |  228.63µs | 185.27µs |             |
|   08 | 149.64µs |  335.00µs |   1.75ms |             |
|   09 | 167.20µs |   56.17µs |  51.99µs |             |
|   10 |  76.43µs |  784.77µs |   2.71ms |             |
|   11 |          |           |          |             |
|   12 |          |           |          |             |
|   13 |          |           |          |             |
|   14 |          |           |          |             |
|   15 |          |           |          |             |
|   16 |          |           |          |             |
|   17 |          |           |          |             |
|   18 |          |           |          |             |
|   19 |          |           |          |             |
|   20 |          |           |          |             |
|   21 |          |           |          |             |
|   22 |          |           |          |             |
|   23 |          |           |          |             |
|   24 |          |           |          |             |
|   25 |          |           |          |             |
