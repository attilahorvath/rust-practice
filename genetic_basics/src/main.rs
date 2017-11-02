extern crate genetic_basics;

use genetic_basics::Population;

fn main() {
    let mut population = Population::new();
    population.calculate_fitness();

    println!("{:?}", population);

    for _ in 1..31 {
        population = population.evolve();
        population.calculate_fitness();
        // println!("{:?}", population);
    }

    println!("{:?}", population);
}
