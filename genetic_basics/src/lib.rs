extern crate rand;

use rand::Rng;

const POPULATION_SIZE: usize = 15;
const GENES: usize = 5;

#[derive(Debug)]
struct Phenotype {
    chromosome: String,
    fitness: Option<u64>,
}

impl Phenotype {
    fn new() -> Self {
        let chromosome = (1..(GENES + 1))
            .map(|_| if rand::thread_rng().gen() { '1' } else { '0' })
            .collect();

        Phenotype {
            chromosome,
            fitness: None,
        }
    }

    fn from_chromosome(chromosome: &str) -> Self {
        Phenotype {
            chromosome: chromosome.into(),
            fitness: None,
        }
    }

    fn calculate_fitness(&mut self) {
        let mut n = 0;
        for (index, digit) in self.chromosome.chars().rev().enumerate() {
            n += if digit == '1' { 1 } else { 0 } * 2u64.pow(index as u32);
        }
        self.fitness = Some(n)
    }
}

#[derive(Debug)]
pub struct Population {
    phenotypes: Vec<Phenotype>,
}

fn crossover((a, b): (char, char)) -> (char, char) {
    if rand::thread_rng().gen_range(1, 101) > 70 {
        (a, b)
    } else {
        (b, a)
    }
}

fn mutate(gene: char) -> char {
    if rand::thread_rng().gen_range(1, 101) > 99 {
        if gene == '1' { '0' } else { '1' }
    } else {
        gene
    }
}

impl Population {
    pub fn new() -> Self {
        let mut phenotypes = Vec::with_capacity(POPULATION_SIZE);

        for _ in 1..(POPULATION_SIZE + 1) {
            phenotypes.push(Phenotype::new());
        }

        Population { phenotypes }
    }

    pub fn calculate_fitness(&mut self) {
        for p in self.phenotypes.iter_mut() {
            p.calculate_fitness();
        }
    }

    pub fn evolve(mut self) -> Self {
        self.phenotypes.sort_unstable_by_key(|p| p.fitness);

        let mut new_phenotypes = Vec::with_capacity(POPULATION_SIZE);

        for _ in 1..(POPULATION_SIZE / 2 + 1) {
            let chromosome_a = &self.random_phenotype().chromosome;
            let chromosome_b = &self.random_phenotype().chromosome;

            let (ca, cb): (String, String) = chromosome_a
                .chars()
                .zip(chromosome_b.chars())
                .map(crossover)
                .map(|(a, b)| (mutate(a), mutate(b)))
                .unzip();

            new_phenotypes.push(Phenotype::from_chromosome(&ca));
            new_phenotypes.push(Phenotype::from_chromosome(&cb));
        }

        Population { phenotypes: new_phenotypes }
    }

    fn random_phenotype(&self) -> &Phenotype {
        let sum_fitness = self.phenotypes
            .iter()
            .map(|p| p.fitness.unwrap())
            .sum::<u64>();

        let target_fitness = rand::thread_rng().gen_range(1, sum_fitness + 1);
        let mut current_fitness = 0;

        self.phenotypes
            .iter()
            .take_while(|p| {
                let c = current_fitness;
                current_fitness += p.fitness.unwrap();
                c < target_fitness
            })
            .last()
            .unwrap()
    }
}
