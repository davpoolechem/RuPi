mod methods;

use num;

//-- core engine --//
pub struct Engine {
    pi: f64,
}

//-- constructors --//
impl Engine {
    pub fn new() -> Engine {
        return Engine { pi: 0. };
    }
}

//-- associated functions --//
impl Engine {
    pub fn pi(&self) -> f64 {
        return self.pi;
    }

    pub fn compute<T>(&mut self, num_samples: T) -> ()
    where
        T: num::PrimInt + Into<f64>,
        std::ops::Range<T>: std::iter::Iterator,
    {
        self.pi = methods::simple::compute(num_samples);
    }
}
