use rand::{thread_rng, Rng};
use test::Bencher;
use std::iter;

lazy_static! {
    static ref WIN_SIZE: usize = { 1000 };
    static ref VEC1: Vec<i64> = { (0..1_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
}

fn mavg_windows(ws: usize, v: &Vec<i64>) -> Vec<f64> {
   return v.windows(ws).map(|x| x.iter().sum::<i64>() as f64 / x.len() as f64).collect();
}

fn mavg_for(ws: usize, v: &Vec<i64>) -> Vec<f64> {
    let mut res = Vec::with_capacity(v.len()-ws+1);
    let mut acc:i64 = 0;
    for i in 0..ws-1 {
        acc += v[i];
    }
    for i in ws-1..v.len() {
        acc += v[i];
        res.push(acc as f64 / ws as f64);
        acc -= v[1+i-ws];
    }
    return res;
}

fn mavg_iter(ws: usize, v: &Vec<i64>) -> Vec<f64> {
    let r = v.iter().zip(v.iter().skip(ws-1)).scan(v.iter().take(ws-1).sum::<i64>(), |acc,(&x,&y)| {
        *acc += y;
        let r = Some(*acc as f64 / ws as f64);
        *acc -= x;
        return r;
    }).collect();
    return r;
}

fn q_mavg_iter(ws: usize, v: &Vec<i64>) -> Vec<f64> {
    let r0 = v.iter().take(ws-1).enumerate().scan(0, |acc,(i,&x)| {
        *acc += x;
        return Some(*acc as f64 / (i+1) as f64);
    });
    let r1 = v.iter().zip(v.iter().skip(ws-1)).scan(v.iter().take(ws-1).sum::<i64>(), |acc,(&x,&y)| {
        *acc += y;
        let r = Some(*acc as f64 / ws as f64);
        *acc -= x;
        return r;
    });
    let r = r0.chain(r1).collect();
    return r;
}

fn q_msum(ws: usize, v: &Vec<i64>) -> Vec<i64> {
    let it1:Vec<i64> = v.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    }).collect();
    let it2 = iter::repeat(&0).take(ws).chain(it1.iter()).take(v.len());
    let it3 = it1.iter().zip(it2).map(|(x,y)| x-y);
    return it3.collect();
}

fn q_mavg(ws: usize, v: &Vec<i64>) -> Vec<f64> {
    let ms = q_msum(ws, &v);
    let mc = q_msum(ws, &iter::repeat(1).take(v.len()).collect());
    return ms.iter().zip(mc.iter()).map(|(&x,&y)| x as f64 / y as f64).collect();
}

fn q_mavg_for(ws: usize, v: &Vec<i64>) -> Vec<f64> {
    let mut res = Vec::with_capacity(v.len()-ws+1);
    let mut acc:i64 = 0;
    for i in 0..ws-1 {
        acc += v[i];
        res.push(acc as f64 / (i+1) as f64);
    }
    for i in ws-1..v.len() {
        acc += v[i];
        res.push(acc as f64 / ws as f64);
        acc -= v[1+i-ws];
    }
    return res;
}

#[test]
fn mavg_test() {
    let res = vec![2.0,3.0,4.0];
    let test = vec![1,2,3,4,5];
    assert_eq!(res, mavg_windows(3, &test));
    assert_eq!(res, mavg_for(3, &test));
    assert_eq!(res, mavg_iter(3, &test));
    assert_eq!(vec![1.0, 1.5, 2.0, 3.0, 4.0], q_mavg(3, &test));
    assert_eq!(vec![1.0, 1.5, 2.0, 3.0, 4.0], q_mavg_for(3, &test));
    assert_eq!(vec![1.0, 1.5, 2.0, 3.0, 4.0], q_mavg_iter(3, &test));
}

// #[bench]
fn mavg_windows_bench(b: &mut Bencher) {
    b.iter(|| mavg_windows(*WIN_SIZE, &VEC1) );
}

#[bench]
fn mavg_for_bench(b: &mut Bencher) {
    b.iter(|| mavg_for(*WIN_SIZE, &VEC1) );
}

#[bench]
fn mavg_iter_bench(b: &mut Bencher) {
    b.iter(|| mavg_iter(*WIN_SIZE, &VEC1)) ;
}

#[bench]
fn q_mavg_bench(b: &mut Bencher) {
    b.iter(|| q_mavg(*WIN_SIZE, &VEC1) );
}

#[bench]
fn q_mavg_for_bench(b: &mut Bencher) {
    b.iter(|| q_mavg_for(*WIN_SIZE, &VEC1) );
}

#[bench]
fn q_mavg_iter_bench(b: &mut Bencher) {
    b.iter(|| q_mavg_iter(*WIN_SIZE, &VEC1) );
}

