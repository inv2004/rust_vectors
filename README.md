# rust_vectors
### Benchmarks to compare different languages on vector processing
- Rust
- Q (kdb)
- C (maybe in future)

Intel(R) Core(TM) i5-7200U CPU @ 2.50GHz:
8GB  Physical Memory  2133 \ Kingston (MemoryDevices = ?2? )

_time in ms_

|test           | KDB+ 3.5  | rustc 1.22.0-nightly |
|---------------|:---------:|----------------------|
| deltas        | 6.563     | 3.633                |
| find_odd_mul  | 37.355    | 7,422                |
| amend_odd_mul | 39.888    | 3,535                |
| mavg          | 22.889    | 4,406                |
| q_mavg        | 32.659    | 6,235                |
| max           | 12.22     | 7,273                |


KDB+ 3.5 2017.09.06 \ w32/ 4()core 4095MB:
- deltas:         6.563
- find_odd_mul:       37.355
- find_odd_cmp_mul:   38.879
- amend_odd_mul:      39.888
- mavg (msum):    22.889
- mavg (999_mavg):    34.284
- q_mavg:         32.659
- max:            12.22

rustc 1.22.0-nightly:
- test deltas::deltas_for_bench               ... bench:   4,136,178 ns/iter (+/- 284,732)
- test deltas::deltas_iter_bench              ... bench:   4,118,904 ns/iter (+/- 254,391)
- test deltas::deltas_windows_bench           ... bench:   3,633,447 ns/iter (+/- 88,539)
- test find_replace::find_odd_mul_bench            ... bench:   7,422,591 ns/iter (+/- 531,274)
- test find_replace::amend_odd_mul_bench     ... bench:   3,535,786 ns/iter (+/- 109,961)
- test find_replace::amend_odd_mul_mut_bench ... bench:  12,275,589 ns/iter (+/- 148,298)
- test mavg::mavg_for_bench                   ... bench:   4,406,677 ns/iter (+/- 142,674)
- test mavg::mavg_iter_bench                  ... bench:   6,192,713 ns/iter (+/- 631,387)
- test mavg::mavg_windows_bench               ... bench: 115,744,958 ns/iter (+/- 10,532,016)
- test mavg::q_mavg_bench                     ... bench:  27,077,236 ns/iter (+/- 377,166)
- test mavg::q_mavg_for_bench                 ... bench:   6,235,493 ns/iter (+/- 121,298)
- test mavg::q_mavg_iter_bench                ... bench:   6,826,521 ns/iter (+/- 153,109)
- test other::max_for_bench                   ... bench:   7,273,087 ns/iter (+/- 96,294)
- test other::max_iter_bench                  ... bench:  23,458,570 ns/iter (+/- 165,373)
