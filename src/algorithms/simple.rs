use num;
use rand::Rng;

pub fn compute<T>(num_samples: T) -> f64
where
    T: num::PrimInt + Into<f64>,
    std::ops::Range<T>: std::iter::Iterator,
{
    return 4.0
        * (T::from(
            (T::one()..(num_samples + T::one()))
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
