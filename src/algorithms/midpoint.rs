use rayon::prelude::*;

pub fn compute(num_samples: u128) -> f64 {
    let a = 0.0;
    let b = 1.0;
    let n = num_samples as f64;
    let prefactor = (b - a) / n;

    return 4.0
        * prefactor
        * (1..=num_samples)
            .into_par_iter()
            .map(|k| {
                let mk: f64 = a + (b - a) * (2.0 * (k as f64) - 1.0) / (2.0 * n);
                return (1.0 - mk.powf(2.0)).sqrt();
            })
            .collect::<Vec<f64>>()
            .par_iter()
            .sum::<f64>();
}
