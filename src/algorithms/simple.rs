use rand::Rng;
use rayon::prelude::*;

pub fn compute(num_samples: u128) -> f64
{
    return 4.0*((1_u128..(num_samples + 1_u128))
                .into_par_iter()
                .map(|_| {
                    rand::thread_rng().gen_range(0.0_f64, 1.0_f64).powf(2.0)
                        + rand::thread_rng().gen_range(0.0_f64, 1.0_f64).powf(2.0)
                        <= 1.0
                })
                .into_par_iter()
                .filter(|x| *x)
                .collect::<Vec<bool>>()
                .len()
        as f64)/(num_samples as f64);
}

/*
pub fn compute<'a, T>(num_samples: T) -> f64
where
    T: num::PrimInt + Into<f64>,
    &std::ops::Range<T>: rayon::iter::IntoParallelIterator
{
    return 4.0
        * (T::from(
            (T::one()..(num_samples + T::one()))
                .par_iter()
                .map(|_| {
                    rand::thread_rng().gen_range(0.0_f64, 1.0_f64).powf(2.0)
                        + rand::thread_rng().gen_range(0.0_f64, 1.0_f64).powf(2.0)
                        <= 1.0
                })
                .collect::<Vec<bool>>()
                .into_iter()
                .filter(|x| *x)
                .collect::<Vec<bool>>()
                .len(),
        )
        .unwrap()
        .into()
            / (num_samples.into()));
}
*/
