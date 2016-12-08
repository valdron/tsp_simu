extern crate nalgebra;
extern crate rand;

#[macro_use]
extern crate lazy_static;

mod examples;
mod route;
mod tsp_example;

use route::{Route};
use std::time::{Instant,Duration};
use tsp_example::TspExample;
use examples::*;
use nalgebra::DMatrix;
use std::mem;
use std::clone::Clone;
use rand::{Rng,SeedableRng, StdRng};
use std::thread;
use std::sync::Arc;
use std::fs::{File, OpenOptions};
use std::io::Write;

//  Example 3 = 22 cities Deutschland
//  Example 4 = 48 cities NA
//  Example 5 = 59 cities Deutschland







fn main() {

    let example_3_dist_matrix = DMatrix::from_column_iter(22,22,examples::EXAMPLE_3_DIST_VEC.iter());
    let example_4_dist_matrix = DMatrix::from_column_iter(48,48,examples::EXAMPLE_4_DIST_VEC.iter());
    let example_5_dist_matrix = DMatrix::from_column_iter(59,59,examples::EXAMPLE_5_DIST_VEC.iter());

    let exp3 = Arc::new(TspExample::new(example_3_dist_matrix));
    let exp4 = Arc::new(TspExample::new(example_4_dist_matrix));
    let exp5 = Arc::new(TspExample::new(example_5_dist_matrix));

    let mut param_vec: Vec<(f64, f64, f64)>= vec![
                                                (         100.0,          1.0,          0.99),
                                                (         100.0,          0.1,          0.99),
                                                (         100.0,          0.1,          0.999),
                                                (        1000.0,          0.1,          0.999),
                                                (        1000.0,         0.01,          0.999),
                                                (        1000.0,         0.01,          0.9999),
                                                (       10000.0,         0.01,          0.9999),
                                                (       10000.0,        0.001,          0.9999),
                                                (       10000.0,        0.001,          0.99999),
                                                (      100000.0,        0.001,          0.99999),
                                                (      100000.0,       0.0001,          0.99999),
                                                (      100000.0,       0.0001,          0.999999),
                                                (     1000000.0,       0.0001,          0.999999),
                                                (     1000000.0,      0.00001,          0.999999),
                                                (     1000000.0,      0.00001,          0.9999999),
                                                (     10000000.0,      0.00001,          0.9999999),
                                                (     10000000.0,      0.000001,          0.9999999),
                                                (     10000000.0,      0.000001,          0.99999999),
                                                (     100000000.0,      0.000001,          0.99999999),
                                                (     100000000.0,      0.0000001,          0.99999999),
                                                (     100000000.0,      0.0000001,          0.999999999),
                                                (     1000000000.0,      0.0000001,          0.999999999),
                                                (     1000000000.0,      0.00000001,          0.999999999)];

    for i in param_vec {
        let t_exp3 = exp3.clone();
        let t_exp4 = exp4.clone();
        let t_exp5 = exp5.clone();





        let thread_exp_3 = thread::spawn( move ||{
            let now = Instant::now();
            let tour = t_exp3.simulated_annealing(i.0 , i.1, i.2);
            let mut file_3 = OpenOptions::new().append(true)
                .create(true)
                .open("example3.txt").unwrap();
            let elapsed = now.elapsed();
            write!(&mut file_3,"Example 3 ==> T_High:\t{:?}\tT_Low:\t{:?}\tBeta:\t{:?}\tTime: {}.{}\tLength: {:?}\t Tour:\t{:?}\n",i.0, i.1, i.2, elapsed.as_secs(),elapsed.subsec_nanos(),tour.route_length,tour.route);
        });

        let thread_exp_4 = thread::spawn( move ||{
            let now = Instant::now();
            let tour = t_exp4.simulated_annealing(i.0 , i.1, i.2);
            let mut file_4 = OpenOptions::new().append(true)
                .create(true)
                .open("example4.txt").unwrap();
            let elapsed = now.elapsed();

            write!(&mut file_4,"Example 4 ==> T_High:\t{:?}\tT_Low:\t{:?}\tBeta:\t{:?}\tTime: {}.{}\tLength: {:?}\t Tour:\t{:?}\n",i.0, i.1, i.2, elapsed.as_secs(),elapsed.subsec_nanos(),tour.route_length,tour.route);
        });

        let thread_exp_5 = thread::spawn( move ||{
            let now = Instant::now();
            let tour = t_exp5.simulated_annealing(i.0 , i.1, i.2);
            let mut file_5 = OpenOptions::new().append(true)
                .create(true)
                .open("example5.txt").unwrap();
            let elapsed = now.elapsed();

            write!(&mut file_5,"Example 5 ==> T_High:\t{:?}\tT_Low:\t{:?}\tBeta:\t{:?}\tTime: {}.{}\tLength: {:?}\t Tour:\t{:?}\n",i.0, i.1, i.2, elapsed.as_secs(),elapsed.subsec_nanos(),tour.route_length,tour.route);
        });

        let _ = thread_exp_3.join();
        let _ = thread_exp_4.join();
        let _ = thread_exp_5.join();

    }





}
