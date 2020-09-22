use rupi;

use clap::{Arg, App};
use std::process;

fn main() {
    //-- define input options --//
    let matches = App::new("RuPi")
                          .version("0.1")                                       
                          .author("David Poole <davpoole@iastate.edu>")               
                          .about("Computes digits of pi")                         
                          .arg(Arg::with_name("algorithm")                         
                               .short("a")                                      
                               .long("algorithm")                                  
                               .help("Determines choice of algorithm used")
                               .takes_value(true)                              
                               .help("Determines algorithm used"))
                          .arg(Arg::with_name("INPUT")                          
                               .help("Varies with algoithm; higher means better accuracy")              
                               .required(true)                                  
                               .index(1))    
                          .get_matches();                                       
                                                                                
    //-- process input options --//
    let algorithm: rupi::Algorithm = match matches.value_of("algorithm") { 
        Some("monte_carlo") => rupi::Algorithm::MonteCarlo,          
        Some("monte-carlo") => rupi::Algorithm::MonteCarlo,          
        Some("montecarlo") => rupi::Algorithm::MonteCarlo,          
        Some("midpoint") => rupi::Algorithm::MidPoint,
        Some(_)  => {
            eprintln!("An invalid algorithm has been specified! Exiting program...");
            process::exit(1)
        },
        None => {
            eprintln!("No algorithm has been specified! Defaulting to Monte Carlo.");
            rupi::Algorithm::MonteCarlo
        },
    };

    let num_samples: i32 = matches
        .value_of("INPUT")
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    //-- create engine --//
    let mut engine: rupi::Engine = rupi::Engine::new(algorithm);
    
    //-- compute and print pi --//
    if let Err(e) = engine.compute(num_samples) {
        eprintln!("Error while computing pi: {}", e); 
    } else {
        println!("Pi successfully computed! Value: {}", engine.pi());
    }
}
