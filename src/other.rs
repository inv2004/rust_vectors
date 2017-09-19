use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..10_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn max_iter(v: &Vec<i64>) -> i64 {
    return *v.iter().max().unwrap();
}

fn max_for(v: &Vec<i64>) -> i64 {
    let mut max = 0;
    for x in v.iter() {
        if *x > max {
            max = *x;
        }
    }
    return max;
}

#[test]
fn mdiff_test() {
    assert_eq!(5, max_iter(&vec![1,2,3,5,4]));
    assert_eq!(5, max_for(&vec![1,2,3,5,4]));
}

#[bench]
fn mdiff_iter_bench(b: &mut Bencher) {
    b.iter(|| max_iter(&VEC1) );
}

#[bench]
fn mdiff_for_bench(b: &mut Bencher) {
    b.iter(|| max_for(&VEC1) );
}
