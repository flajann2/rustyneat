/// Base neuron

use std::vec;
use super::*;

pub struct Neuron {
    genes: genome::Genes,
}

pub type Neurons = Vec<Neuron>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron(){
    }
}
