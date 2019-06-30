# rust_vectors
### Benchmarks to compare different languages on vector processing
- Rust
- Q (kdb)
- C (maybe in the future)

_time in ms_

**Intel(R) Core(TM) i5-7500 CPU @ 3.40GHz @ 8GB DDR (2666, Speed = 2400) BLS**

|test           | KDB+ 3.6  | rustc 1.35.0-nightly |
|---------------|:---------:|:--------------------:|
| deltas        | 3.612     | 3,250                |
| dev           | 3.658     | 1,484                |
| odd_mul_amend | 19.596    | 3,532                |
| odd_mul       | 11.537    | 7,451                |
| mavg*         | 8.923     | 3,894                |
| q_mavg*       | 15.612    | 6,923                |
| max           | 6.359     | 5,068                |
| med           | 4.000     | 8,053                |
| wavg          | 46.407    | 8,941                |

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
rustc 1.37.0-nightly (433a46781 2019-06-28)
- test deltas::deltas_for_bench            ... bench:   3,396,580 ns/iter (+/- 156,114)
- test deltas::deltas_iter_bench           ... bench:   3,418,585 ns/iter (+/- 144,242)
- test deltas::deltas_windows_bench        ... bench:   3,250,772 ns/iter (+/- 373,163)
- test dev::dev_for_bench                  ... bench:   1,489,067 ns/iter (+/- 51,052)
- test dev::dev_iter_bench                 ... bench:   1,484,515 ns/iter (+/- 53,891)
- test find_amend::odd_mul_amend_bench     ... bench:   3,532,095 ns/iter (+/- 141,856)
- test find_amend::odd_mul_amend_mut_bench ... bench:  11,917,850 ns/iter (+/- 549,862)
- test find_amend::odd_mul_bench           ... bench:   7,451,385 ns/iter (+/- 188,779)
- test mavg::mavg_for_bench                ... bench:   3,894,140 ns/iter (+/- 159,540)
- test mavg::mavg_iter_bench               ... bench:   6,649,170 ns/iter (+/- 227,757)
- test mavg::mavg_windows_bench            ... bench:  98,630,060 ns/iter (+/- 1,181,047)
- test mavg::q_mavg_bench                  ... bench:  29,621,820 ns/iter (+/- 817,520)
- test mavg::q_mavg_for_bench              ... bench:   6,923,525 ns/iter (+/- 252,576)
- test mavg::q_mavg_iter_bench             ... bench:   7,991,675 ns/iter (+/- 285,502)
- test max::max_for_bench                  ... bench:   5,068,010 ns/iter (+/- 146,638)
- test max::max_iter_bench                 ... bench:   5,665,025 ns/iter (+/- 201,529)
- test med::med_lib_bench                  ... bench:   8,053,405 ns/iter (+/- 222,563)
- test med::med_sort_bench                 ... bench:  15,260,220 ns/iter (+/- 380,961)
- test wavg::wavg_for_bench                ... bench:   8,941,910 ns/iter (+/- 340,732)
- test wavg::wavg_iter_bench               ... bench:  11,838,760 ns/iter (+/- 467,774)
