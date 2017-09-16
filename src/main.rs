#![feature(test)]
#![feature(type_ascription)]
#[macro_use]

extern crate lazy_static;
extern crate test;
extern crate rand;

mod tests {
	use rand::{thread_rng, Rng};
	use test::Bencher;
    lazy_static! {
    	static ref VEC1: Vec<i64> = { (0..10_000_000).map(|_| thread_rng().gen_range(0,1000)).collect() };
    }

    // len: 10m ; q32: 65ms , rust: 41ms
    // #[bench] // find deltas
    fn deltas_windows(b: &mut Bencher) {
        b.iter(|| {
            let r:Vec<i64> = VEC1.windows(2).map(|x| x[1] - x[0]).collect();
            return r;
        });
    }

    // let: 10m q32: 65ms , rust: 41ms
    // #[bench] // find deltas
    fn deltas_zip(b: &mut Bencher) {
        b.iter(|| {
            let r:Vec<i64> = VEC1.iter().zip(VEC1.iter().skip(1)).map(|(x,y)| x-y).collect();
            return r;
        });
    }

    // let: 10m q32: 65ms , rust: 51ms
    // #[bench] // find deltas
    fn deltas_iter(b: &mut Bencher) {
        b.iter(|| {
            let mut res:Vec<i64> = Vec::with_capacity(10_000_000);
            for i in 0..(VEC1.len()-1) {
                res.push(VEC1[i+1] - VEC1[i]);
            }
            return res;
        });
    }

    // let: 10m q32: 367ms , rust: 83ms
    #[bench] // multiple odd numbers with 100
    fn odd_mul_100_filter_map(b: &mut Bencher) {
        b.iter(|| {
            let r = VEC1.iter().filter(|x| *x%2 == 0).map(|x| x*100).collect::<Vec<i64>>();
            return r;
        });
    }

    // let: 10m q32: 367ms, rust: 39ms
    #[bench] // multiple odd numbers with 100
    fn odd_mul_100_map(b: &mut Bencher) {
        b.iter(|| {
            let r = VEC1.iter().map(|x| if x%2 == 0 { x*100 } else { *x }).collect::<Vec<i64>>();
            return r;
        });
    }

    // let: 10m q32: 367ms, rust: 39ms
    #[bench] // multiple odd numbers with 100
    fn odd_mul_100_for(b: &mut Bencher) {
        b.iter(|| {
            let mut a = VEC1;
            for i in VEC1. {
                if i%2 == 0 {
                    *i = *i*100;
                }
            }
        });
    }

}
