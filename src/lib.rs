extern crate nalgebra as na;

use std::fmt::{Debug, Display, Formatter};
use std::ops::Mul;
use rand::Rng;

type ActivationFunction = dyn Fn(f32) -> f32;

#[derive(Debug)]
pub enum Error {
    Default
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => { write!(f, "Unknown Error")}
        }
    }
}

impl std::error::Error for Error {

}

pub struct CogWheel {
    weights: Vec<na::DMatrix<f32>>,
    activation_function: Box<ActivationFunction>
}

impl Debug for CogWheel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for w in &self.weights {
            str.push_str( &format!("{}", w));
        }
        write!(f, "{}", str)
    }
}

impl CogWheel {

    pub fn new(layer_shapes : Vec<usize>, activation_function : Option<Box<ActivationFunction>>) -> Self {
        let mut weights = Vec::new();

        for i in 0..layer_shapes.len()-1 {
            weights.push(na::DMatrix::zeros(layer_shapes[i], layer_shapes[i+1]));
        }

        let activation_function = activation_function.unwrap_or( {
           Box::from( | t : f32 | {
                1.0 / ( 1.0 + std::f32::consts::E.powf(-t) )
            })
        });

        CogWheel {
            weights,
            activation_function
        }
    }

    pub fn init_random_weights(&mut self) {
        let mut rng = rand::thread_rng();
        for w in self.weights.iter_mut() {
            for mut col in w.column_iter_mut() {
                for v in col.iter_mut() {
                    *v = rng.gen();
                }
            }
        }
    }

    pub fn run(&self, input: na::DVector<f32> ) -> Result<na::DVector<f32>, Error> {
        let mut output = input;
        for i in 0..self.weights.len() {
            println!("{}{}", i, &output);
            output = self.apply_weights(&output, i)?;
        }
        Ok(output)
    }

    pub fn apply_weights(&self, input: &na::DVector<f32>, idx : usize ) -> Result<na::DVector<f32>, Error> {
        let wm = self.weights.get(idx).ok_or(Error::Default)?;
        if wm.shape().0 != input.shape().0 {
            return Err(Error::Default);
        }

        let output = input.transpose().mul( wm);
        let output : na::DVector<f32> = output.map(|x| (self.activation_function)(x)).transpose().column(0).try_into().or(Err(Error::Default))?;

        Ok(output)
    }
}

