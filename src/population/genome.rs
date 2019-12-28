/// Genome
use std::vec;
use super::*;

pub struct Gene<I = u32, W = f32> {
    input: I,
    output: I,
    innovation: I,
    weight: W,
    enabled: bool,
    distal: bool,
}

pub type Genes<I = u32, W = f32> = Vec<Gene<I, W>>;

pub struct Genome<Score = f32, I = u32, W = f32> {
    genes: Genes<I, W>,
    score: Score,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene() {
        // assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_genome() {
        // assert_eq!(bad_add(1, 2), 3);
    }
}
