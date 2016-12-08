use nalgebra::DMatrix;
use std::mem;
use std::clone::Clone;
use rand::{Rng,SeedableRng, StdRng};
use route::Route;

#[derive(Debug, Clone)]
pub struct TspExample<'a> {
    dist_matrix: DMatrix<&'a usize>,
    cities: usize,
}

impl<'a> TspExample<'a> {
    pub fn new(dist_matrix: DMatrix<&'a usize>) -> TspExample<'a> {
        let cities = dist_matrix.ncols();
        TspExample {
            dist_matrix: dist_matrix,
            cities: cities
        }
    }

    pub fn simulated_annealing(&self, t_high: f64, t_low: f64, beta: f64) -> Route {

        let mut temp = t_high;
        let mut current_route = Route::new(&self.dist_matrix);
        let seed: &[usize] = &[0usize];
        let mut rng:  StdRng = SeedableRng::from_seed(seed);

        while temp > t_low {

            let mut i = (rng.next_f64() * self.cities as f64).floor() as usize;
            let mut j = (rng.next_f64() * self.cities as f64).floor() as usize;

            if i > j {
                mem::swap(&mut i, &mut j);
            }

            let mut new_route = current_route.clone();
            new_route.swap_at(i,j);
            new_route.update_length();
            let z: f64 = rng.next_f64();

            let p: f64 = if new_route.route_length > current_route.route_length {
                (-((new_route.route_length-current_route.route_length) as f64)/temp).exp()
            } else {
                1f64
            };

            if z < p {
                current_route = new_route;
            }
            temp *= beta;

        }

        current_route
    }


}