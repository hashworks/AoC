# Advent of Code 2020

https://adventofcode.com/2020/

Solutions will be done in Rust. All part times include individual parsing, except parts where it made no sense at all to parse twice (marked with `*`).

## Runtimes (i5-6600K)

|  Day |  Part 1 | Part 2 | Total | perf stat -r 1000 |
| ---: | ------: | -----: | ----: | ----------------: |
|    1 |    27µs |  131µs | 158µs |             906µs |
|    2 |   128µs |  147µs | 298µs |            1100µs |
|   3* |    56µs |    7µs |  73µs |             717µs |
|   4* |   108µs |  143µs | 258µs |             856µs |
|    5 |    75µs |   71µs | 153µs |             802µs |
|    6 |    66µs |   79µs | 152µs |             730µs |
|    7 |   384µs |  322µs | 715µs |            1329µs |
|    8 |    40µs |   30µs |  80µs |             664µs |
|    9 |    96µs |  799µs | 903µs |            1538µs |
|  10* |    15µs |    1µs |  16µs |             596µs |
|   11 | 16000µs |   TODO |  TODO |              TODO |
|   12 |    27µs |   17µs |  50µs |             632µs |
|   13 |    TODO |   TODO |  TODO |              TODO |
|   14 |    TODO |   TODO |  TODO |              TODO |
|   15 |    TODO |   TODO |  TODO |              TODO |
|  16* |   166µs |  193µs | 363µs |             958µs |