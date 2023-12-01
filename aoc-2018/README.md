# Advent of Code 2018

## Current benchmarks as [this commit](ff06bfb8d7aaca6f367f55273d748e85e79f7baa)
```
test day1::tests::d1_bench_ex1  ... bench:      26,292 ns/iter (+/- 17,496)
test day1::tests::d1_bench_ex2  ... bench:   7,588,990 ns/iter (+/- 549,303)

test day2::tests::d2_bench_ex1    ... bench:     117,360 ns/iter (+/- 20,767)
test day2::tests::d2_bench_ex2    ... bench:     824,857 ns/iter (+/- 72,092)

test day3::tests::d3_ex1_bench_sl ... bench:   1,434,067 ns/iter (+/- 154,992)
test day3::tests::d3_ex2_bench    ... bench:   1,105,170 ns/iter (+/- 389,014)

test day4::tests::d4_ex1_bench  ... bench:     107,835 ns/iter (+/- 3,063)
test day4::tests::d4_ex2_bench  ... bench:     106,873 ns/iter (+/- 2,001)

// Without improvement
test day5::tests::d5_bench_ex1 ... bench:  19,436,470 ns/iter (+/- 1,987,872)
test day5::tests::d5_bench_ex2 ... bench:  76,633,060 ns/iter (+/- 3,325,339)

// With small improvement
test day5::tests::d5_bench_ex1 ... bench:     224,975 ns/iter (+/- 57,992)
test day5::tests::d5_bench_ex2 ... bench:   3,225,215 ns/iter (+/- 773,984)

// With the lookback version of my algorithm that is way better readable. (sort of stolen btw)
test day5::tests::d5_bench_ex1    ... bench:     165,003 ns/iter (+/- 25,646)
test day5::tests::d5_bench_ex2    ... bench:   1,926,270 ns/iter (+/- 149,976)

test day6::tests::d6_bench_ex1  ... bench:  16,484,510 ns/iter (+/- 1,410,523)
test day6::tests::d6_bench_ex2  ... bench:  14,281,040 ns/iter (+/- 323,689)

test day7::tests::d7_bench_ex1  ... bench:       3,901 ns/iter (+/- 47)
test day7::tests::d7_bench_ex2  ... bench:       4,600 ns/iter (+/- 366)

test day8::tests::d8_bench_ex1 ... bench:     251,076 ns/iter (+/- 14,854)
test day8::tests::d8_bench_ex2 ... bench:     446,193 ns/iter (+/- 71,412)

test day9::tests::d9_bench_ex1 ... bench:     398,390 ns/iter (+/- 363,164)
test day9::tests::d9_bench_ex2 ... bench:  61,729,740 ns/iter (+/- 14,717,271)

test day10::tests::d10_bench_ex1 ... bench:   2,718,090 ns/iter (+/- 28,040)

test day11::tests::day11_bench_ex1 ... bench:     249,310 ns/iter (+/- 5,213)
test day11::tests::day11_bench_ex2 ... bench:  11,484,850 ns/iter (+/- 155,478)

test day12::tests::day12_ex1 ... bench:      16,288 ns/iter (+/- 446)
test day12::tests::day12_ex2 ... bench:      71,002 ns/iter (+/- 734)

test day13::tests::day13_bench_ex1  ... bench:     193,311 ns/iter (+/- 66,861)
test day13::tests::day13_bench_ex2  ... bench:  12,626,100 ns/iter (+/- 1,091,917)
test day13::tests::day13_bench_read ... bench:     170,154 ns/iter (+/- 5,594)

test day14::tests::day14_bench_ex1 ... bench:   4,342,005 ns/iter (+/- 47,718)
test day14::tests::day14_bench_ex2 ... bench: 168,918,650 ns/iter (+/- 3,944,586)

test day15::tests::day15_bench_ex1 ... bench:  21,467,650 ns/iter (+/- 2,138,195)
test day15::tests::day15_bench_ex2 ... bench:  59,974,530 ns/iter (+/- 533,632)

test day16::tests::day16_bench_ex1 ... bench:      71,706 ns/iter (+/- 3,447)
test day16::tests::day16_bench_ex2 ... bench:     714,985 ns/iter (+/- 81,466)

test day17::tests::day17_bench_ex1 ... bench:   2,517,960 ns/iter (+/- 211,187)
test day17::tests::day17_bench_ex2 ... bench:   2,547,990 ns/iter (+/- 193,829)

test day18::tests::day18_bench_ex1     ... bench:  95,925,590 ns/iter (+/- 10,913,841)
test day18::tests::day18_bench_ex1_cpu ... bench:     378,960 ns/iter (+/- 6,394)
test day18::tests::day18_bench_ex2     ... bench: 148,032,800 ns/iter (+/- 14,556,801)
test day18::tests::day18_bench_ex2_cpu ... bench:  19,804,000 ns/iter (+/- 379,598)

test day20::tests::day20_bench_ex12 ... bench:     960,695 ns/iter (+/- 50,461)
test day20::tests::day20_bench_read ... bench:     358,315 ns/iter (+/- 50,063)

// No like for this day, hardcoded for my input
test day21::tests::day21_bench_ex1 ... bench:       2,936 ns/iter (+/- 248)
test day21::tests::day21_bench_ex2 ... bench:  20,973,627 ns/iter (+/- 1,109,644)

test day22::tests::day22_bench_ex1 ... bench:     292,180 ns/iter (+/- 4,120)
test day22::tests::day22_bench_ex2 ... bench: 568,798,470 ns/iter (+/- 16,193,117)

test day23::tests::day23_bench_ex1 ... bench:       4,625 ns/iter (+/- 4,363)
test day23::tests::day23_bench_ex2 ... bench:     160,979 ns/iter (+/- 84,587)

test day24::tests::day24_bench_ex1  ... bench:   1,801,101 ns/iter (+/- 80,831)
test day24::tests::day24_bench_ex2  ... bench:  25,806,550 ns/iter (+/- 466,207)
test day24::tests::day24_bench_read ... bench:      14,596 ns/iter (+/- 683)

test day25::tests::day25_bench_ex1 ... bench:     995,515 ns/iter (+/- 66,186)

```
