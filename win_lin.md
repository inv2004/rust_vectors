**CPU: i5-6300U**

**ubuntu 18.03**
```
test deltas::deltas_for_bench            ... bench:   1,125,897 ns/iter (+/- 44,497)
test deltas::deltas_iter_bench           ... bench:   1,125,435 ns/iter (+/- 46,285)
test deltas::deltas_windows_bench        ... bench:     967,241 ns/iter (+/- 8,545)
test dev::dev_for_bench                  ... bench:     777,883 ns/iter (+/- 27,158)
test dev::dev_iter_bench                 ... bench:   1,965,383 ns/iter (+/- 76,701)
test find_amend::odd_mul_amend_bench     ... bench:   1,600,583 ns/iter (+/- 55,602)
test find_amend::odd_mul_amend_mut_bench ... bench:  14,209,441 ns/iter (+/- 209,517)
test find_amend::odd_mul_bench           ... bench:   6,272,499 ns/iter (+/- 172,004)
test mavg::mavg_for_bench                ... bench:   1,577,655 ns/iter (+/- 102,695)
test mavg::mavg_iter_bench               ... bench:   1,792,805 ns/iter (+/- 51,607)
test mavg::mavg_windows_bench            ... bench: 118,376,891 ns/iter (+/- 269,206)
test mavg::q_mavg_bench                  ... bench:  18,554,411 ns/iter (+/- 694,868)
test mavg::q_mavg_for_bench              ... bench:   1,885,389 ns/iter (+/- 169,542)
test mavg::q_mavg_iter_bench             ... bench:   2,841,406 ns/iter (+/- 57,482)
test max::max_for_bench                  ... bench:   6,521,068 ns/iter (+/- 10,120)
test max::max_iter_bench                 ... bench:   6,996,199 ns/iter (+/- 117,731)
test med::med_for_bench                  ... bench:     512,669 ns/iter (+/- 48,635)
test wavg::wavg_for_bench                ... bench:  11,924,135 ns/iter (+/- 246,425)
test wavg::wavg_iter_bench               ... bench:  14,368,884 ns/iter (+/- 38,400)
```

**windows-10**
```
test deltas::deltas_for_bench            ... bench:   5,456,710 ns/iter (+/- 1,802,048)
test deltas::deltas_iter_bench           ... bench:   5,423,970 ns/iter (+/- 196,594)
test deltas::deltas_windows_bench        ... bench:   5,475,610 ns/iter (+/- 2,155,559)
test dev::dev_for_bench                  ... bench:     672,445 ns/iter (+/- 52,974)
test dev::dev_iter_bench                 ... bench:   1,835,950 ns/iter (+/- 62,762)
test find_amend::odd_mul_amend_bench     ... bench:   5,555,555 ns/iter (+/- 124,631)
test find_amend::odd_mul_amend_mut_bench ... bench:  15,608,370 ns/iter (+/- 225,956)
test find_amend::odd_mul_bench           ... bench:   9,836,110 ns/iter (+/- 178,418)
test mavg::mavg_for_bench                ... bench:   5,937,830 ns/iter (+/- 48,384)
test mavg::mavg_iter_bench               ... bench:  10,885,310 ns/iter (+/- 239,815)
test mavg::mavg_windows_bench            ... bench: 118,535,380 ns/iter (+/- 679,579)
test mavg::q_mavg_bench                  ... bench:  45,372,690 ns/iter (+/- 843,246)
test mavg::q_mavg_for_bench              ... bench:   5,812,670 ns/iter (+/- 119,222)
test mavg::q_mavg_iter_bench             ... bench:  11,423,580 ns/iter (+/- 339,376)
test max::max_for_bench                  ... bench:   7,447,270 ns/iter (+/- 3,108,026)
test max::max_iter_bench                 ... bench:   6,802,900 ns/iter (+/- 158,484)
test med::med_for_bench                  ... bench:     452,720 ns/iter (+/- 12,022)
test wavg::wavg_for_bench                ... bench:  10,813,640 ns/iter (+/- 223,904)
test wavg::wavg_iter_bench               ... bench:  14,924,200 ns/iter (+/- 923,622)
```
