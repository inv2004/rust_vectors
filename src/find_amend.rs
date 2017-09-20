use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn odd_mul(v: &Vec<i64>) -> Vec<i64> {
    return v.iter().filter(|&x| 1 == x % 2).map(|x| x*100).collect();
}

fn odd_mul_amend(v: &Vec<i64>) -> Vec<i64> {
    return v.iter().map(|&x| {
        if 1 == x%2 {
            x * 100
        } else {
            x
        }
    }).collect();
}

fn odd_mul_amend_mut(v: &Vec<i64>) -> Vec<i64> {
    let res = &mut v.clone();
    for x in res.iter_mut() {
        if 1 == *x%2 {
            *x *= 100;
        }
    }
    return res.to_vec();
}

#[test]
fn odd_mul_test() {
    assert_eq!(vec![100,300,500], odd_mul(&vec![1,2,3,4,5]));
    assert_eq!(vec![100,2,300,4,500], odd_mul_amend(&vec![1,2,3,4,5]));
    assert_eq!(vec![100,2,300,4,500], odd_mul_amend_mut(&vec![1,2,3,4,5]));
}

#[bench]
fn odd_mul_bench(b: &mut Bencher) {
    b.iter(|| odd_mul(&VEC1) );
}

#[bench]
fn odd_mul_amend_bench(b: &mut Bencher) {
    b.iter(|| odd_mul_amend(&VEC1) );
}

#[bench]
fn odd_mul_amend_mut_bench(b: &mut Bencher) {
    b.iter(|| odd_mul_amend_mut(&VEC1) );
}
