# Advent of Code 2022

https://adventofcode.com/2022/

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

## Results

Please note that the following numbers are relative, and should only be used to compare the performance between commits and days.

|  Day |  Parsing |    Part 1 |     Part 2 |                          Notes |
| ---: | -------: | --------: | ---------: | -----------------------------: |
|   01 |  94.66µs |    2.48µs |     8.16µs |                                |
|   02 |  81.82µs |    9.46µs |     8.57µs |                                |
|   03 |  33.23µs |   16.40µs |    16.04µs |                                |
|   04 |  64.73µs |    1.08µs |     0.66µs |                                |
|   05 |  63.22µs |    7.91µs |    12.85µs |                                |
|   06 |   8.99µs |    5.05µs |    13.65µs |                                |
|   07 | 100.38µs |   68.85µs |    63.84µs |                                |
|   08 |  14.75µs |  204.94µs |   215.83µs |                                |
|   09 |  73.35µs |  218.36µs |   198.87µs |                                |
|   10 |  12.73µs |    0.58µs |     9.41µs |                                |
|   11 |  15.69µs |   12.72µs |     5.75µs |                                |
|   12 |  18.54µs |  366.54µs |   342.14µs |                                |
|   13 | 330.05µs |    5.31µs |    70.18µs |                                |
|   14 | 336.84µs | 1329.54µs | 53189.44µs | This can be reduced with a DFS |
|   15 |          |           |            |                                |
|   16 |          |           |            |                                |
|   17 |          |           |            |                                |
|   18 |          |           |            |                                |
|   19 |          |           |            |                                |
|   20 |          |           |            |                                |
|   21 |          |           |            |                                |
|   22 |          |           |            |                                |
|   23 |          |           |            |                                |
|   24 |          |           |            |                                |
|   25 |          |           |            |                                |
