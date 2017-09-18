#![feature(test)]
#![feature(type_ascription)]
#![feature(iterator_for_each)]
#[macro_use]

extern crate lazy_static;
extern crate test;
extern crate rand;

// mod mavg;
// mod deltas;
mod update;

//     // // let: 10m q32: 367ms , rust: 83ms
//     // #[bench] // multiple odd numbers with 100
//     // fn odd_mul_100_filter_map(b: &mut Bencher) {
//     //     b.iter(|| {
//     //         let r = VEC1.iter().filter(|x| *x%2 == 0).map(|x| x*100).collect::<Vec<i64>>();
//     //         return r;
//     //     });
//     // }

//     // // let: 10m q32: 367ms, rust: 39ms
//     // #[bench] // multiple odd numbers with 100
//     // fn odd_mul_100_map(b: &mut Bencher) {
//     //     b.iter(|| {
//     //         let r = VEC1.iter().map(|x| if x%2 == 0 { x*100 } else { *x }).collect::<Vec<i64>>();
//     //         return r;
//     //     });
//     // }

//     // // let: 10m q32: 367ms, rust: 39ms
//     // #[bench] // multiple odd numbers with 100
//     // fn odd_mul_100_for(b: &mut Bencher) {
//     //     b.iter(|| {
//     //         let mut a = VEC1;
//     //         for i in a.iter_mut() {
//     //             if *i%2 == 0 {
//     //                 *i = *i*100;
//     //             }
//     //         }
//     //         return VEC1;
//     //     });
//     // }

