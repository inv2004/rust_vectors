# rust_vectors
### Benchmarks to compare different languages on vector processing
- Rust
- Q (kdb)
- C (maybe in the future)

_time in ms_

**Intel(R) Core(TM) i5-7500 CPU @ 3.40GHz @ 8GB DDR (2666, Speed = 2400) BLS**

|test           | KDB+ 3.6  | rustc 1.35.0-nightly | rustc 1.21.1-stable | nim 1.0.4-danger |
|---------------|:---------:|:--------------------:|:-------------------:|:----------------:|
| deltas        | 3.612     | 3,250                | 3,627               | 3.176            |
| dev           | 3.658     | 1,484                | 0,587               | 0.634            |
| odd_mul_amend | 19.596    | 3,532                | 3,795               | 5.287            |
| odd_mul       | 11.537    | 7,451                | 7,595               | 7.930            |
| mavg*         | 8.923     | 3,894                | 4,161               | 2.539            |
| q_mavg*       | 15.612    | 6,923                | 3,968               | 2.396            |
| max           | 6.359     | 5,068                | 5,189               | 0.000002 ???     |
| med           | 4.000     | 8,053                | 0,330               | 0.000002 ???     |
| wavg          | 46.407    | 8,941                | 9,171               | 2.132            |

\* _window-size: 1000_
---
KDB+ 3.6 2019.03.07 Copyright (C) 1993-2019 Kx Systems w64/ 4(16)core 16339MB
- deltas: 3.612
- dev: 3.658
- odd_mul_amend: 19.596
- odd_mul: 18.4
- odd_mul_cmp: 11.537
- mavg (msum): 8.923
- mavg (999_): 16.452
- q_mavg: 15.612
- max: 6.359
- med: 4
- wavg: 46.407

---
rustup 1.21.1 (7832b2ebe 2019-12-20) with -d:danger option
- test deltas::deltas_for_bench            ... bench:   3,650,765 ns/iter (+/- 413,176)
- test deltas::deltas_iter_bench           ... bench:   3,627,175 ns/iter (+/- 312,758)
- test deltas::deltas_windows_bench        ... bench:   3,556,115 ns/iter (+/- 477,610)
- test dev::dev_for_bench                  ... bench:     587,526 ns/iter (+/- 50,748)
- test dev::dev_iter_bench                 ... bench:   1,504,820 ns/iter (+/- 85,999)
- test find_amend::odd_mul_amend_bench     ... bench:   3,795,840 ns/iter (+/- 203,972)
- test find_amend::odd_mul_amend_mut_bench ... bench:  12,605,180 ns/iter (+/- 794,094)
- test find_amend::odd_mul_bench           ... bench:   7,595,520 ns/iter (+/- 438,932)
- test mavg::mavg_for_bench                ... bench:   4,161,535 ns/iter (+/- 248,397)
- test mavg::mavg_iter_bench               ... bench:   7,200,470 ns/iter (+/- 347,317)
- test mavg::mavg_windows_bench            ... bench:  99,228,060 ns/iter (+/- 1,172,131)
- test mavg::q_mavg_bench                  ... bench:  30,783,650 ns/iter (+/- 1,196,841)
- test mavg::q_mavg_for_bench              ... bench:   3,968,735 ns/iter (+/- 283,278)
- test mavg::q_mavg_iter_bench             ... bench:   8,301,150 ns/iter (+/- 350,548)
- test max::max_for_bench                  ... bench:   5,189,480 ns/iter (+/- 260,692)
- test max::max_iter_bench                 ... bench:   5,831,495 ns/iter (+/- 265,318)
- test med::med_for_bench                  ... bench:     330,475 ns/iter (+/- 24,759)
- test wavg::wavg_for_bench                ... bench:   9,171,995 ns/iter (+/- 1,065,234)
- test wavg::wavg_iter_bench               ... bench:  11,740,730 ns/iter (+/- 826,348)

---
Nim Compiler Version 1.0.4 [Windows: amd64] Compiled at 2019-11-27
- Benchmark: deltas_bench(x = 0) Time: 3.1764ms ± 47.8762us
- Benchmark: dev_1_bench(x = 0) Time: 6.9362ms ± 111.3220us
- Benchmark: dev_2_bench(x = 0) Time: 4.1151ms ± 51.7601us
- Benchmark: dev_3_bench(x = 0) Time: 2.1248ms ± 23.6025us
- Benchmark: dev_4_bench(x = 0) Time: 634.2516us ± 13.5936us
- Benchmark: odd_mul_amend_1_bench(x = 0) Time: 5.2874ms ± 49.1023us
- Benchmark: odd_mul_amend_2_bench(x = 0) Time: 5.7551ms ± 107.6175us
- Benchmark: odd_mul_1_bench(x = 0) Time: 7.9304ms ± 65.6060us
- Benchmark: odd_mul_2_bench(x = 0) Time: 9.5562ms ± 54.5145us
- Benchmark: mavg_1_bench(x = 0) Time: 5.8697ms ± 44.3138us
- Benchmark: mavg_2_bench(x = 0) Time: 2.5399ms ± 28.7444us
- Benchmark: mavg_3_bench(x = 0) Time: 4.1420ms ± 57.0274us
- Benchmark: q_mavg_bench(x = 0) Time: 2.3967ms ± 31.9291us
- Benchmark: max_1_bench(x = 0) Time: 2.2931ns ± 6.2839ns
- Benchmark: max_2_bench(x = 0) Time: 6.4741ms ± 53.9765us
- Benchmark: med_1_bench(x = 0) Time: 2.0146ns ± 3.6758ns
- Benchmark: med_2_bench(x = 0) Time: 62.5124ms ± 263.8609us
- Benchmark: wavg_bench(x = 0) Time: 2.1326ns ± 3.7926ns


