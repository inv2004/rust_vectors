use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..10_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
    static ref VEC2: Vec<i64> = { (0..10_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn wavg_iter(v1: &Vec<i64>, v2: &Vec<i64>) -> f64 {
    v2.iter().zip(v1.iter()).map(|(x,y)| x*y).sum::<i64>() as f64 / v1.iter().sum::<i64>() as f64
}

fn wavg_for(v1: &Vec<i64>, v2: &Vec<i64>) -> f64 {
    let mut acc = 0;
    let mut acc_weight = 0;
    for i in 0..v1.len() {
        acc += v1[i]*v2[i];
        acc_weight += v1[i];
    }
    return acc as f64 / acc_weight as f64;
}

#[test]
fn wavg_test() {
    assert_eq!(3.125, wavg_iter(&vec![1,2,1,3,1], &vec![1,2,3,4,5]));
    assert_eq!(3.125, wavg_for(&vec![1,2,1,3,1], &vec![1,2,3,4,5]));
}

#[bench]
fn wavg_iter_bench(b: &mut Bencher) {
    b.iter(|| wavg_iter(&VEC1, &VEC2) );
}

#[bench]
fn wavg_for_bench(b: &mut Bencher) {
    b.iter(|| wavg_for(&VEC1, &VEC2) );
}
