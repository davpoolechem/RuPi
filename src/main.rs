use rupi;

#[macro_use]
extern crate clap;
use clap::App;

use std::process;

fn main() {
    //-- define input options --//
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //-- process input options --//
    let algorithm: rupi::Algorithm = match matches.value_of("algorithm") {
        Some("monte_carlo") => rupi::Algorithm::MonteCarlo,
        Some("monte-carlo") => rupi::Algorithm::MonteCarlo,
        Some("montecarlo") => rupi::Algorithm::MonteCarlo,
        Some("midpoint") => rupi::Algorithm::MidPoint,
        Some(_) => {
            eprintln!("Error while processing input: invalid algorithm");
            process::exit(1);
        }
        None => {
            eprintln!("No algorithm has been specified! Defaulting to Monte Carlo.");
            rupi::Algorithm::MonteCarlo
        }
    };

    let num_samples: u128 = matches
        .value_of("INPUT")
        .unwrap()
        .trim()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Error while processing input: INPUT must be an integer.");
            process::exit(1);
        });

    if num_samples <= 0 {
        eprintln!("Error while processing input: INPUT must be greater than 0.");
        process::exit(1);
    }

    //-- create engine --//
    let mut engine: rupi::Engine = rupi::Engine::new(algorithm);

    //-- compute and print pi --//
    if let Err(e) = engine.compute(num_samples) {
        eprintln!("Error while computing pi: {}", e);
    } else {
        println!("Pi successfully computed! Value: {}", engine.pi());
    }
}
