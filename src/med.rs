use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn med_for(v: &Vec<i64>) -> f64 {
    let mut res = 0;
    for x in v {
        res += x;
    }
    res as f64 / v.len() as f64
}

#[test]
fn med_test() {
    assert_eq!(3.0, med_for(&vec![1,2,3,5,4]));
    assert_eq!(3.5, med_for(&vec![1,2,3,5,4,6]));
}

#[bench]
fn med_for_bench(b: &mut Bencher) {
    b.iter(|| med_for(&VEC1) );
}
