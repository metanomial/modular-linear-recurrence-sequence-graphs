use std::{
    // collections::{HashMap, HashSet},
    num::NonZeroUsize,
    path::PathBuf,
    process,
};

use clap::Parser;

#[cfg(test)]
mod tests;

/// Calculate and render directed graphs from modular linear recurrence
/// sequences with constant coefficients.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Sequence modulus. Must be a non-zero positive integer.
    #[clap(value_parser)]
    modulus: NonZeroUsize,

    /// Constant coefficients of the recurrence relation. Must be a list of
    /// colon-separated non-negative integers. The coefficient's index
    /// corresponds to the n-minus-i-th term in the recurrence relation.
    /// The default value is "1:1"
    #[clap(value_parser)]
    coefficients: Option<String>,

    /// Render a mosaic image from sequence pairs and output at the given path.
    /// Only available for recurrence relations of first or second order.
    #[clap(short, long)]
    render: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let coefficients = parse_coefficients(cli.coefficients);

    if cli.render.is_some() && coefficients.len() != 2 {
        eprintln!("Mosaic rendering is only supported for second order recurse relations");
        process::exit(1);
    }

    // let mut cycle_number = 0;
    // for i in 0..modulus {
    //     for j in 0..modulus {
    //         if found.contains(&(i, j)) {
    //             continue;
    //         }
    //         let mut cycle = Vec::<u32>::new();
    //         let mut a = i;
    //         let mut b = j;
    //         while !found.contains(&(a, b)) {
    //             found.insert((a, b));
    //             cycle.push(a);
    //             pairs.insert((a, b), cycle_number);
    //             let new = (a + b) % modulus;
    //             a = b;
    //             b = new;
    //         }
    //         cycle_number += 1;
    //         cycles.push(cycle);
    //     }
    // }
}

fn parse_coefficients(string: Option<String>) -> Vec<usize> {
    match string {
        Some(string) => {
            let mut coefficients = Vec::<usize>::new();
            for sym in string.split(":") {
                match sym.trim().parse() {
                    Ok(i) => coefficients.push(i),
                    Err(e) => {
                        eprintln!("Invalid coefficient \"{}\": {}", sym, e);
                        process::exit(1)
                    }
                }
            }
            coefficients
        }
        None => vec![1, 1],
    }
}

// fn map_pairs(modulus: usize, coefficients: Vec<usize>) -> HashMap<Vec<usize>, usize> {
//     let mut pairs = HashMap::<Vec<usize>, usize>::new();
//     pairs
// }

// fn print_cycles(modulus: u32) {
//     println!("Base {}", modulus);

//     let cycles = CycleSet::compute(modulus).cycles;
//     let mut cycle_lengths: Vec<usize> = cycles.iter().map(|c| c.len()).collect();
//     cycle_lengths.sort();

//     println!("Cycle count: {}", cycles.len());
//     println!(
//         "Cycle lengths: {}",
//         cycle_lengths
//             .iter()
//             .map(|l| l.to_string())
//             .collect::<Vec<String>>()
//             .join(", ")
//     );

//     for cycle in cycles {
//         println!(
//             "[{}]: {}",
//             cycle.len(),
//             cycle
//                 .iter()
//                 .map(|n| n.to_string())
//                 .collect::<Vec<String>>()
//                 .join(" ")
//         );
//     }
// }

// fn render_pairs(modulus: u32, output: PathBuf) {
//     println!("Base {}", modulus);

//     let cycle_set = CycleSet::compute(modulus);
//     let cycles = cycle_set.cycles;
//     let pairs = cycle_set.pairs;

//     println!("Cycle count: {}", cycles.len());

//     todo!();
// }
