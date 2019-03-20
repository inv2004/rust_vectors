# rust_vectors
### Benchmarks to compare different languages on vector processing
- Rust
- Q (kdb)
- C (maybe in the future)

_time in ms_

**Intel(R) Core(TM) i5-7500 CPU @ 3.40GHz @ 8GB DDR (2666, Speed = 2400) BLS**

|test           | KDB+ 3.6  | rustc 1.35.0-nightly |
|---------------|:---------:|:--------------------:|
| deltas        | 3.612     | 3,311                |
| dev           | 3.658     | 1,509                |
| odd_mul_amend | 19.596    | 3,549                |
| odd_mul       | 11.537    | 7,488                |
| mavg*         | 8.923     | 3,919                |
| q_mavg*       | 15.612    | 7,100                |
| max           | 6.359     | 5,091                |
| med           | 4.000     | 8,209                |
| wavg          | 46.407    | 9,246                |

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
rustc 1.35.0-nightly (3eb4890df 2019-03-19)
- test deltas::deltas_for_bench            ... bench:   3,314,476 ns/iter (+/- 435,366)
- test deltas::deltas_iter_bench           ... bench:   3,311,907 ns/iter (+/- 409,272)
- test deltas::deltas_windows_bench        ... bench:   3,470,931 ns/iter (+/- 503,709)
- test dev::dev_for_bench                  ... bench:   1,510,616 ns/iter (+/- 247,306)
- test dev::dev_iter_bench                 ... bench:   1,509,129 ns/iter (+/- 222,551)
- test find_amend::odd_mul_amend_bench     ... bench:   3,549,113 ns/iter (+/- 466,748)
- test find_amend::odd_mul_amend_mut_bench ... bench:  12,042,389 ns/iter (+/- 1,156,833)
- test find_amend::odd_mul_bench           ... bench:   7,488,130 ns/iter (+/- 714,152)
- test mavg::mavg_for_bench                ... bench:   3,919,426 ns/iter (+/- 576,403)
- test mavg::mavg_iter_bench               ... bench:   7,555,315 ns/iter (+/- 887,485)
- test mavg::mavg_windows_bench            ... bench: 100,580,626 ns/iter (+/- 4,216,948)
- test mavg::q_mavg_bench                  ... bench:  29,324,482 ns/iter (+/- 2,251,996)
- test mavg::q_mavg_for_bench              ... bench:   7,100,300 ns/iter (+/- 854,337)
- test mavg::q_mavg_iter_bench             ... bench:  10,567,445 ns/iter (+/- 1,040,029)
- test max::max_for_bench                  ... bench:   5,091,827 ns/iter (+/- 500,324)
- test max::max_iter_bench                 ... bench:   6,318,674 ns/iter (+/- 415,726)
- test med::med_lib_bench                  ... bench:   8,209,076 ns/iter (+/- 591,117)
- test med::med_sort_bench                 ... bench:  15,651,806 ns/iter (+/- 1,044,719)
- test wavg::wavg_for_bench                ... bench:   9,246,597 ns/iter (+/- 883,381)
- test wavg::wavg_iter_bench               ... bench:  12,044,071 ns/iter (+/- 1,307,341)
