//! # Neuron
//! Neuron knows what other neurons it has inputs from, and
//! what presents as outputs. A neuron can have zero or more
//! inputs and outputs.


use super::*;
use std::vec;

pub struct Neuron {

}

pub type Neurons = Vec<Neuron>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron() {}
}
