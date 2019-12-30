use super::*;
/// Critter construction
use std::vec;

pub struct Critter<'g> {
    genome: genome::Genome,
    neurons: neuron::Neurons<'g>,
}

trait Sex<G> {
    fn cotius(&self, other: G) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_critter() {}
}
