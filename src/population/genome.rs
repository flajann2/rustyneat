use super::*;
/// Genome
use std::{vec, option};

pub struct Gene<I = u32, W = f32> {
    input: I,
    output: I,
    innovation: I,
    weight: W,
    enabled: bool,
    distal: bool,
}

impl Gene {
    fn new() -> Self {
        return Self{input: 0,
                    output: 0,
                    innovation: 0,
                    weight: 0f32,
                    enabled: false,
                    distal: false};
    }
}

pub type Genes<I = u32, W = f32> = Vec<Gene<I, W>>;

pub struct Genome<Score = f32, I = u32, W = f32> {
    genes: Genes<I, W>,
    score: Score,
}

impl Genome {
    fn new() -> Self {
        return Genome{score: 0f32, genes: Genes::new()}
    }
    
    fn add(&mut self, gene: Gene) -> Option<&Gene> {
        self.genes.push(gene);
        return self.genes.last();
    }
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
        let mut gn = Genome::new();
        let g1 = Gene {
            enabled: true,
            output: 1,
            ..Gene::new()
        };
        let g2 = Gene {
            input: 2,
            output: 1,
            ..Gene::new()
        };
        assert_ne!(g1.enabled, g2.enabled);
        let bg1 = gn.add(g1);
        let bg2 = gn.add(g2);
        assert_eq!(gn.genes.len(), 2);
        
        match bg1 {
            Some(b1) => assert_eq!(b1.output, 1),
            None => assert!(false, "should never get here")
        }
    }
}
