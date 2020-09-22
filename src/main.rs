use rupi;
use std::env;

fn main() {
    //-- accept input arguments --//
    let argc = env::args().len();
    let argv: Vec<String> = env::args().collect();

    let num_samples: i32 = argv[1].trim().parse().unwrap();

    //-- create engine --//
    let mut engine: rupi::Engine = rupi::Engine::new(rupi::Algorithm::MonteCarlo);
    //let mut engine: rupi::Engine = rupi::Engine::new(rupi::Algorithm::MidPoint);
    
    //-- compute and print pi --//
    if let Err(e) = engine.compute(num_samples) {
        println!("Error while computing pi: {}", e); 
    } else {
        println!("Pi successfully computed! Value: {}", engine.pi());
    }
}
