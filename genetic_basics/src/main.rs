extern crate genetic_basics;

use genetic_basics::Population;

fn max_digits(chromosome: &String) -> u64 {
    chromosome
        .chars()
        .rev()
        .enumerate()
        .map(|(index, digit)| {
            (if digit == '1' { 1 } else { 0 }) * 2u64.pow(index as u32)
        })
        .sum()
}

fn min_digits(chromosome: &String) -> u64 {
    2u64.pow(chromosome.len() as u32) - max_digits(chromosome)
}

fn exact_digits(chromosome: &String) -> u64 {
    "1100000000000000000000011"
        .chars()
        .zip(chromosome.chars())
        .map(|(a, b)| (a == b) as u64)
        .sum()
}

fn main() {
    let mut population = Population::new();

    for _ in 1..51 {
        population.calculate_fitness(&exact_digits);
        println!("{}", population);
        population = population.evolve();
    }
}
