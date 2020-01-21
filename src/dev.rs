use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn dev_iter(v: &Vec<i64>) -> f64 {
    let avg = v.iter().sum::<i64>() as f64 / v.len() as f64;
    let s = v.iter().map(|&x| (x as f64 - avg).powi(2)).sum::<f64>() / v.len() as f64;
    return s.sqrt();
}

fn dev_for(v: &Vec<i64>) -> f64 {
    let len = v.len() as f64;
    let mut sum1 = 0;
    let mut sum2 = 0;
    for x in v {
        sum1 += x;
        sum2 += x*x;
    }
    (sum2 as f64 / len as f64 - (sum1*sum1) as f64 /(len*len) as f64).sqrt()
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

