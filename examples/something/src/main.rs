use std::time::Instant;

use rand::{Rng, thread_rng};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator, ParallelSliceMut};

fn main() {
    // let a = AtomicU8::new(0);
    // a.fetch_sub(1, SeqCst);
    // println!("{}", a.load(SeqCst));
    //
    // let nums = vec![1, 2, 3, 4, 5];
    // let squares: Vec<_> = nums.par_iter().map(|&i| i * i).collect();
    // println!("{:?}", squares);
    //
    // let data: Vec<i32> = (1..=10000).collect();
    // let result: Vec<i32> = data
    //     .par_iter()
    //     .filter(|&&x| x % 2 == 0)
    //     .map(|&x| x * x)
    //     .collect();
    // println!("res: {:?}", result);

    // let mut vec = vec![0; 100_000_000];
    // vec.par_iter_mut().for_each(|p| {
    //     let mut rng = thread_rng();
    //     *p = rng.gen_range(0..1000)
    // });
    // let timer = Instant::now();
    // vec.par_sort(); // Result: 10.818203838s
    // // vec.sort(); // Result: 42.330823364s
    // println!("Result: {:?}", timer.elapsed());

    // let threads = 5;
    // let pool = ThreadPoolBuilder::new().num_threads(threads).build().expect("");

    let vec1 = vec![0; 10_000_000];
    let vec2 = vec![0; 10_000_000];
    let vec3 = vec![0; 10_000_000];
    let mut vec_arr = vec![vec1, vec2, vec3];
    for x in vec_arr.iter_mut() {
        fill_vec(x);
    }

    vec_arr.into_par_iter().map(|mut vec| {
        println!("{}", thread_id::get());
        let timer = Instant::now();
        vec.par_sort();
        // vec.sort();
        println!("Result: {:?}", timer.elapsed());
    }).count();
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = rng.gen_range(0..1000)
    });
}
