use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..10_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn dev_iter(v: &Vec<i64>) -> f64 {
    let avg = v.iter().sum::<i64>() as f64 / v.len() as f64;
    let s = v.iter().map(|&x| (x as f64 - avg).powi(2)).sum::<f64>() / v.len() as f64;
    return s.sqrt();
}

fn dev_for(v: &Vec<i64>) -> f64 {
    let len = v.len() as f64;
    let mut acc = 0;
    for x in v {
        acc += *x;
    }
    let avg = acc as f64 / len;
    let mut acc = 0.0;
    for x in v {
        acc += (*x as f64 - avg).powi(2);
    }
    acc /= len;
    return acc.sqrt();
}

#[test]
fn dev_test() {
    assert_eq!(1.4142135623730951, dev_iter(&vec![1,2,3,4,5]));
    assert_eq!(1.4142135623730951, dev_for(&vec![1,2,3,4,5]));
}

#[bench]
fn dev_iter_bench(b: &mut Bencher) {
    b.iter(|| dev_iter(&VEC1));
}

#[bench]
fn dev_for_bench(b: &mut Bencher) {
    b.iter(|| dev_for(&VEC1));
}

