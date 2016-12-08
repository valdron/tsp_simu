
use std::ops::{Index, IndexMut};
use rand::{StdRng, Rng};

pub trait Fitness {
    fn get_fitness(&self) -> f64;
}

pub trait Crossover<P = Self> {
    fn crossover(&self, partner: &P) -> P;
}

pub trait Mutation {
    fn mutate(&self, mutation_rate: f64) -> Self;
}

pub trait GenIndividual {
    fn gen_individuum() -> Self;
}

#[derive(Debug)]
pub struct Population<T>
    where T: Fitness + Crossover + Mutation + GenIndividual + Clone
{
    population: Vec<T>,
    pop_size:   usize,
    tournament_size: usize,
    mutation_rate: f64,
    elitism: bool,
}

impl<T> IndexMut<usize> for Population<T>
where T: Fitness + Crossover + Mutation + GenIndividual + Clone {

    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T{
        &mut self.population[index]
    }

}

impl<T> Index<usize> for Population<T>
where T: Fitness + Crossover + Mutation + GenIndividual + Clone {

    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T{
        &self.population[index]
    }

}

impl<T> Population<T>
    where T: Fitness + Crossover + Mutation + GenIndividual + Clone {

    pub fn new(size: usize, tournament_size: usize, mutation_rate: f64, elitism: bool) -> Result<Population<T>, String> {
        if size == 0usize {
            return Err("Size must be greater than 0".to_string());
        }
        Ok(Population{
            tournament_size: tournament_size,
            mutation_rate: mutation_rate,
            elitism: elitism,
            population: Vec::with_capacity(size),
            pop_size: size,
        })
    }

    pub fn init(mut self) -> Population<T> {
        for i in 0..self.pop_size-1 {
            self.population.push(T::gen_individuum());
        }
        self
    }

    pub fn get_fittest(&self) -> T {
        let mut highest_fitness = 0f64;
        let mut index_fittest = 0;
        for i in 0..self.pop_size-1 {
            if self.population[i].get_fitness() > highest_fitness {
                index_fittest = i;
                highest_fitness = self.population[i].get_fitness();
            }
        }
        self.population[index_fittest].clone()
    }

    pub fn get_pop_size(&self) -> usize {
        self.pop_size
    }

    pub fn tournament_selection(&self) -> T {
        let mut tournament: Population<T> = Population::new(self.tournament_size,0,0f64,false).unwrap();
        let mut rng:  StdRng = StdRng::new().unwrap();
        for _ in 0..self.tournament_size-1 {
            tournament.population.push(self.population[(rng.next_f64() * self.tournament_size as f64) as usize].clone());
        }

        tournament.get_fittest()
    }

    pub fn evolve_population(&self) -> Population<T> {
        let mut new_pop: Population<T> = Population::new(self.pop_size,self.tournament_size,self.mutation_rate,self.elitism).unwrap();
        let elitismoffset = if self.elitism {
            1
        } else {
            0
        };
        if self.elitism {
            new_pop.population.push(self.get_fittest());
        }

        for _ in elitismoffset..new_pop.pop_size-1{
            let parent1: T = self.tournament_selection();
            let parent2: T = self.tournament_selection();

            let child: T = parent1.crossover(&parent2).mutate(new_pop.mutation_rate);

            new_pop.population.push(child);
        }
        new_pop
    }
}

