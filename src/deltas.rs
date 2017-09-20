use rand::{thread_rng, Rng};
use test::Bencher;

lazy_static! {
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

// fn deltas_for<'a, T>(v: &'a Vec<T>) -> Vec< <&'a T as std::ops::Sub>::Output >
//     where &'a T: std::ops::Sub
// fn deltas_for<T>(v: &Vec<T>) -> Vec<T>
//     where T: Copy + ops::Sub<Output=T>
fn deltas_for(v: &Vec<i64>) -> Vec<i64>
{
    let mut res = Vec::with_capacity(v.len()-1);
    for i in 1..v.len() {
        res.push(&v[i] - &v[i-1]);
    }
    return res;
}

fn deltas_iter(v: &Vec<i64>) -> Vec<i64> {
    v.iter().zip(v.iter().skip(1)).map(|(x,y)| y-x).collect()
}

fn deltas_windows(v: &Vec<i64>) -> Vec<i64> {
    v.windows(2).map(|x| x[1]-x[0]).collect()
}

#[test]
fn deltas_test() {
    assert_eq!(vec![1,-2,4,1], deltas_for(&vec![1,2,0,4,5]));
    assert_eq!(vec![1,-2,4,1], deltas_iter(&vec![1,2,0,4,5]));
    assert_eq!(vec![1,-2,4,1], deltas_windows(&vec![1,2,0,4,5]));
}

#[bench]
fn deltas_for_bench(b: &mut Bencher) {
    b.iter(|| deltas_for(&VEC1));
}

#[bench]
fn deltas_iter_bench(b: &mut Bencher) {
    b.iter(|| deltas_for(&VEC1));
}

#[bench]
fn deltas_windows_bench(b: &mut Bencher) {
    b.iter(|| deltas_windows(&VEC1) );
}
