mod pi;
use std::env;

fn main() {
    //-- accept input arguments --//
    let argc = env::args().len();
    let argv: Vec<String> = env::args().collect();

    let num_samples: i32 = argv[1].trim().parse().unwrap();

    //-- compute pi --//
    let mut engine: pi::Engine = pi::Engine::new(pi::Algorithm::MonteCarlo);
    engine.compute(num_samples);

    //-- print results --//
    println!("Pi: {}", engine.pi());
}
