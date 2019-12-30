//! # Neuron
//! Neuron knows what other neurons it has inputs from, and
//! what presents as outputs. A neuron can have zero or more
//! inputs and outputs.


use super::*;
use std::vec;

/// We have borrowed references to these genes
/// and these are only the genes that has the
/// particular neuron as its output.
type NGenes<'g, I = u32, W = f32> = Vec<&'g mut genome::Gene<I, W>>;

pub struct Neuron<'g> {
    ngenes: NGenes<'g>,
}

pub type Neurons<'g> = Vec<Neuron<'g>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron() {}
}
