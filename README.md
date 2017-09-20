# rust_vectors
### Benchmarks to compare different languages on vector processing
- Rust
- Q (kdb)
- C (maybe in future)

_time in ms_

**Intel(R) Core(TM) i5-7200U CPU @ 2.50GHz
8GB DDR4-SDRAM \ 2133 Kingston (MemoryBanks = 1) \ windows-10**

|test           | KDB+ 3.5  | rustc 1.22.0-nightly |
|---------------|:---------:|:--------------------:|
| deltas        | 6.459     | 3.681                |
| dev           | 14.245    | 1,918                |
| odd_mul_amend | 39.127    | 3,694                |
| odd_mul       | 36.691    | 7,459                |
| mavg*         | 22.157    | 4,277                |
| q_mavg*       | 32.663    | 6,759                |
| max           | 12.223    | 7,587                |
| med           | 7.144     | 8,268                |
| wavg          | 176.389   | 14,137               |

\* _window-size: 1000_

**AMD**
@TODO

KDB+ 3.5 2017.09.06 \ w32/ 4()core 4095MB:
- deltas: 6.459
- dev: 14.245
- odd_mul_amend: 39.127
- odd_mul: 36.691
- odd_mul_cmp: 38.849
- mavg (msum): 22.157
- mavg (999_): 34.261
- q_mavg: 32.663
- max: 12.223
- med: 7.144
- wavg: 176.389

rustc 1.22.0-nightly:
- test deltas::deltas_for_bench            ... bench:   4,066,418 ns/iter (+/- 120,299)
- test deltas::deltas_iter_bench           ... bench:   4,081,956 ns/iter (+/- 98,582)
- test deltas::deltas_windows_bench        ... bench:   3,681,625 ns/iter (+/- 144,432)
- test dev::dev_for_bench                  ... bench:   1,920,586 ns/iter (+/- 78,047)
- test dev::dev_iter_bench                 ... bench:   1,918,320 ns/iter (+/- 51,239)
- test find_amend::odd_mul_amend_bench     ... bench:   3,694,954 ns/iter (+/- 159,949)
- test find_amend::odd_mul_amend_mut_bench ... bench:  12,809,220 ns/iter (+/- 183,225)
- test find_amend::odd_mul_bench           ... bench:   7,459,496 ns/iter (+/- 259,062)
- test mavg::mavg_for_bench                ... bench:   4,277,297 ns/iter (+/- 185,890)
- test mavg::mavg_iter_bench               ... bench:   6,675,940 ns/iter (+/- 188,407)
- test mavg::mavg_windows_bench            ... bench: 116,027,006 ns/iter (+/- 432,662)
- test mavg::q_mavg_bench                  ... bench:  29,325,005 ns/iter (+/- 7,219,800)
- test mavg::q_mavg_for_bench              ... bench:   6,759,612 ns/iter (+/- 270,089)
- test mavg::q_mavg_iter_bench             ... bench:   7,293,153 ns/iter (+/- 203,325)
- test max::max_for_bench                  ... bench:   7,587,591 ns/iter (+/- 98,771)
- test max::max_iter_bench                 ... bench:  23,592,301 ns/iter (+/- 99,220)
- test med::med_lib_bench                  ... bench:   8,268,464 ns/iter (+/- 738,685)
- test med::med_sort_bench                 ... bench: 841,599,381 ns/iter (+/- 3,660,163)
- test wavg::wavg_for_bench                ... bench:  14,137,928 ns/iter (+/- 180,555)
- test wavg::wavg_iter_bench               ... bench:  20,302,925 ns/iter (+/- 250,498)
