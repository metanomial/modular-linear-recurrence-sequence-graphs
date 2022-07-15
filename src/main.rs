use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
};

use clap::Parser;

#[cfg(test)]
mod tests;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Print the cycles
    #[clap(subcommand)]
    program: Program,
}

#[derive(clap::Subcommand, Debug)]
enum Program {
    /// Compute the cycles of all Lucas sequences modulo N
    Cycles {
        #[clap(short, long, value_parser)]
        modulus: u32,
    },

    /// Render the pairs of all Lucas sequences modulo N, colored by associated cycle.
    Render {
        #[clap(short, long, value_parser)]
        modulus: u32,

        /// Image output path
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.program {
        Program::Cycles { modulus } => print_cycles(modulus),
        Program::Render { modulus, output } => {
            let output = output.unwrap_or(env::current_dir().unwrap());
            render_pairs(modulus, output);
        }
    }
}

fn print_cycles(modulus: u32) {
    println!("Base {}", modulus);

    let cycles = CycleSet::compute(modulus).cycles;
    let mut cycle_lengths: Vec<usize> = cycles.iter().map(|c| c.len()).collect();
    cycle_lengths.sort();

    println!("Cycle count: {}", cycles.len());
    println!(
        "Cycle lengths: {}",
        cycle_lengths
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    for cycle in cycles {
        println!(
            "[{}]: {}",
            cycle.len(),
            cycle
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn render_pairs(modulus: u32, output: PathBuf) {
    println!("Base {}", modulus);

    let cycle_set = CycleSet::compute(modulus);
    let cycles = cycle_set.cycles;
    let pairs = cycle_set.pairs;

    println!("Cycle count: {}", cycles.len());

    todo!();
}

struct CycleSet {
    cycles: Vec<Vec<u32>>,
    pairs: HashMap<(u32, u32), usize>,
}

impl CycleSet {
    fn compute(modulus: u32) -> CycleSet {
        let mut found = HashSet::<(u32, u32)>::new();
        let mut pairs = HashMap::<(u32, u32), usize>::new();
        let mut cycles = Vec::<Vec<u32>>::new();
        let mut cycle_number = 0;

        for i in 0..modulus {
            for j in 0..modulus {
                if found.contains(&(i, j)) {
                    continue;
                }
                let mut cycle = Vec::<u32>::new();
                let mut a = i;
                let mut b = j;
                while !found.contains(&(a, b)) {
                    found.insert((a, b));
                    cycle.push(a);
                    pairs.insert((a, b), cycle_number);
                    let new = (a + b) % modulus;
                    a = b;
                    b = new;
                }
                cycle_number += 1;
                cycles.push(cycle);
            }
        }

        CycleSet { cycles, pairs }
    }
}
