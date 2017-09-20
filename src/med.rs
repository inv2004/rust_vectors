extern crate order_stat;

use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_001).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn med_sort(v: &Vec<i64>) -> f64 {
    let len = v.len();
    let mut v_sorted = v.clone();
    v_sorted.sort();
    let middle = len / 2;
    if 0 == len % 2 {
        return (v_sorted[middle-1] + v_sorted[middle]) as f64 / 2.0_f64;
    } else {
        return v_sorted[middle] as f64;
    }
}

fn med_lib(v: &Vec<i64>) -> f64 {
    let len = v.len();
    let middle = len / 2;
    let vv = &mut v.clone();
    if 0 == len % 2 {
        return (*order_stat::kth(vv, middle-1) + *order_stat::kth(vv, middle)) as f64 / 2.0_f64
    } else {
        return *order_stat::kth(vv, middle) as f64;
    }
}

#[test]
fn med_test() {
    assert_eq!(3.0, med_sort(&vec![1,2,3,5,4]));
    assert_eq!(3.5, med_sort(&vec![1,2,3,5,4,6]));
    assert_eq!(3.0, med_lib(&vec![1,2,3,5,4]));
    assert_eq!(3.5, med_lib(&vec![1,2,3,5,4,6]));
}

// #[bench]
// fn med_sort_bench(b: &mut Bencher) {
//     b.iter(|| med_sort(&VEC1) );
// }

#[bench]
fn med_lib_bench(b: &mut Bencher) {
    b.iter(|| med_lib(&VEC1) );
}
