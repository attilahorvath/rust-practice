extern crate rand;

use std::fmt;
use rand::Rng;

const POPULATION_SIZE: usize = 15;
const GENES: usize = 25;

#[derive(Debug, Clone)]
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

    fn calculate_fitness(&mut self, fitness_function: &Fn(&String) -> u64) {
        self.fitness = Some(fitness_function(&self.chromosome));
    }
}

impl fmt::Display for Phenotype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.chromosome, self.fitness.unwrap())
    }
}

#[derive(Debug)]
pub struct Population {
    generation: u32,
    phenotypes: Vec<Phenotype>,
    worst_phenotype_index: Option<usize>,
    best_phenotype_index: Option<usize>,
}

fn crossover((a, b): (char, char)) -> char {
    if rand::thread_rng().gen_range(1, 101) > 70 {
        b
    } else {
        a
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

        Population {
            generation: 0,
            phenotypes,
            worst_phenotype_index: None,
            best_phenotype_index: None,
        }
    }

    pub fn calculate_fitness(&mut self, fitness_function: &Fn(&String) -> u64) {
        for p in self.phenotypes.iter_mut() {
            p.calculate_fitness(fitness_function);
        }

        self.worst_phenotype_index = self.phenotypes
            .iter()
            .enumerate()
            .min_by_key(|&(_, p)| p.fitness.unwrap())
            .map(|(i, _)| i);

        self.best_phenotype_index = self.phenotypes
            .iter()
            .enumerate()
            .max_by_key(|&(_, p)| p.fitness.unwrap())
            .map(|(i, _)| i);
    }

    pub fn evolve(&self) -> Self {
        let phenotypes = (1..(POPULATION_SIZE + 1))
            .enumerate()
            .map(|(index, _)| {
                if index == 0 {
                    return self.best_phenotype().unwrap().clone();
                }

                let chromosome_a = &self.random_phenotype().chromosome;
                let chromosome_b = &self.random_phenotype().chromosome;

                let chromosome = chromosome_a
                    .chars()
                    .zip(chromosome_b.chars())
                    .map(crossover)
                    .map(mutate)
                    .collect::<String>();

                Phenotype::from_chromosome(&chromosome)
            })
            .collect();

        Population {
            generation: self.generation + 1,
            phenotypes,
            worst_phenotype_index: None,
            best_phenotype_index: None,
        }
    }

    fn random_phenotype(&self) -> &Phenotype {
        let sum_fitness = self.phenotypes
            .iter()
            .map(|p| p.fitness.unwrap())
            .sum::<u64>();

        let target_fitness = rand::thread_rng().gen_range(0, sum_fitness + 1);

        if target_fitness == 0 {
            return &self.phenotypes[0];
        }

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

    fn worst_phenotype(&self) -> Option<&Phenotype> {
        self.worst_phenotype_index.map(|i| self.phenotypes.get(i).unwrap())
    }

    fn best_phenotype(&self) -> Option<&Phenotype> {
        self.best_phenotype_index.map(|i| self.phenotypes.get(i).unwrap())
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generation {}: {} - {}",
            self.generation,
            self.worst_phenotype().unwrap(),
            self.best_phenotype().unwrap(),
        )
    }
}
