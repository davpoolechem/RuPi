mod algorithms;

use num;

//-----------------//
//-- core engine --//
//-----------------//
pub struct Engine {
    pi: f64,
    algorithm: Algorithm,
}

//-- constructors --//
impl Engine {
    pub fn new(algorithm: Algorithm) -> Engine {
        return Engine { pi: 0., algorithm };
    }
}

//-- associated functions --//
impl Engine {
    pub fn pi(&self) -> f64 {
        return self.pi;
    }

    pub fn compute<T>(&mut self, num_samples: T) -> Result<(), &'static str>
    where
        T: num::PrimInt + Into<f64>,
        std::ops::Range<T>: std::iter::Iterator,
    {
        match self.algorithm {
            Algorithm::MonteCarlo => {
                return Ok(self.pi = algorithms::simple::compute(num_samples));
            },
            _ => Err("invalid algorithm chosen"),
        }
    }
}

//--------------------//
//-- algorithm enum --//
//--------------------//
pub enum Algorithm {
    MonteCarlo,
    MidPoint,
}
