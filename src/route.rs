use nalgebra::DMatrix;
use std::mem;
use std::clone::Clone;
use rand::{Rng, StdRng};
//use population::{Fitness,Crossover,Mutation,GenIndividual};
use examples::*;



#[derive(Debug, Clone)]
pub struct Route<'b> {
    pub dist_matrix: &'b DMatrix<&'b usize>,
    pub route: Vec<usize>,
    pub route_length: usize,
    pub cities_nr: usize,
}

impl<'b> Route<'b> {
    pub fn new(dist_matrix: &'b DMatrix<&usize>) -> Route<'b> {
        let cities = dist_matrix.ncols();
        let mut start_route: Vec<usize> = (0..cities).collect();


        let mut route = Route{
            dist_matrix: dist_matrix,
            route: start_route,
            route_length: 0,
            cities_nr: cities
        };

        route.update_length();
        route


    }

     pub fn new_w_tour(dist_matrix: &'b DMatrix<&usize>, route_vec: Vec<usize>) -> Route<'b>{

        let cities = dist_matrix.ncols();

        let mut route = Route{
            dist_matrix: dist_matrix,
            route: route_vec,
            route_length: 0,
            cities_nr: cities
        };

        route.update_length();
        route
    }

    pub fn swap_at(&mut self, i: usize, j: usize)  {
        &mut self.route[i..j].reverse();
    }

    pub fn update_length(&mut self) {
        let mut res: usize = 0;
        for i in 0..self.route.len()-1 {
            res += *self.dist_matrix[(self.route[i],self.route[i+1])];
        }
        res += *self.dist_matrix[(self.route[0],self.route[self.route.len() -1])];
        self.route_length = res;
    }

}
/*
impl<'b> Crossover for Route <'b>{
    fn crossover(&self, partner: &Route<'b>) -> Route<'b> {
        let mut rng:  StdRng = StdRng::new().unwrap();
        let mut start_pos = (rng.next_f64() * self.cities_nr as f64) as usize;
        let mut end_pos = (rng.next_f64() * self.cities_nr as f64) as usize;

        if end_pos < start_pos {
            mem::swap(&mut start_pos, &mut end_pos)
        }

        let mut child_vec: Vec<usize> = vec![];

        for i in 0..self.cities_nr {
            if i < end_pos && i >= start_pos{
                child_vec.push(self.route[i]);
            } else {
                child_vec.push(0usize);
            }

        }


        let mut child_vec_counter = 0;
        for i in 0..partner.cities_nr {
            if child_vec_counter == start_pos {
                child_vec_counter = end_pos;
            }
            if child_vec_counter >= child_vec.len() {
                break;
            }
            if !child_vec.contains(&partner.route[i]) {
                child_vec[child_vec_counter] = partner.route[i];

                child_vec_counter += 1;

            }
        }
        Route::new_w_tour(self.dist_matrix,child_vec)
    }
}

impl<'b> Fitness for Route <'b>{
    fn get_fitness(&self) -> f64 {
        1f64/(self.route_length as f64)
    }
}

impl<'b> Mutation for Route <'b> {
    fn mutate(&self, mutation_rate: f64) -> Route<'b> {
        let mut rng:  StdRng = StdRng::new().unwrap();
        let mut mutated_route =  self.clone();

        for i in 0..self.cities_nr-1 {
            if rng.next_f64() < mutation_rate {
                let swap_with = (rng.next_f64() * self.cities_nr as f64) as usize;
                if swap_with > i {
                    mutated_route.swap_at(i, swap_with);
                }else{
                mutated_route.swap_at(swap_with,i);

                }
            }
        }
        mutated_route
    }
}

impl<'b> GenIndividual for Route<'b>{
    fn gen_individuum() -> Route<'b>{

        lazy_static!{
            static ref MATRIX3: DMatrix<&'static usize> = DMatrix::from_column_iter(22,22,EXAMPLE_3_DIST_VEC.iter());
            static ref MATRIX4: DMatrix<&'static usize> = DMatrix::from_column_iter(48,48,EXAMPLE_4_DIST_VEC.iter());
            static ref MATRIX5: DMatrix<&'static usize> = DMatrix::from_column_iter(59,59,EXAMPLE_5_DIST_VEC.iter());
        }

        let mut rng:  StdRng = StdRng::new().unwrap();
        let mut route = Route::new(&MATRIX5);

        rng.shuffle(route.route.as_mut());
        route

    }
}
*/