use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn mul_odd(v: &Vec<i64>) -> Vec<i64> {
    return v.iter().filter(|&x| 1 == x % 2).map(|x| x*100).collect();
}

fn mul_odd_update(v: &Vec<i64>) -> Vec<i64> {
    return v.iter().map(|&x| {
        if 1 == x%2 {
            x * 100
        } else {
            x
        }
    }).collect();
}

#[test]
fn mul_odd_test() {
    assert_eq!(vec![100,300,500], mul_odd(&vec![1,2,3,4,5]));
    assert_eq!(vec![100,2,300,4,500], mul_odd_update(&vec![1,2,3,4,5]));
}

#[bench]
fn mul_odd_bench(b: &mut Bencher) {
    b.iter(|| mul_odd(&VEC1) );
}

#[bench]
fn mul_odd_update_bench(b: &mut Bencher) {
    b.iter(|| mul_odd_update(&VEC1) );
}
