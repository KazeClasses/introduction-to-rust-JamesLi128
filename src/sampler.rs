use rand::{prelude::Distribution, rngs::StdRng, Rng, SeedableRng};
use statrs::distribution::MultivariateNormal;

pub struct State<const N_DIM: usize> {
    // Fill this
    rng: StdRng,
    pub arr: [f64; N_DIM],
    proposal_distribution: MultivariateNormal,
}

fn log_likelihood<const N_DIM: usize>(arr: &[f64]) -> f64 {
    // Fill this
    let mut rst : f64 = 0.0;
    for i in 0..N_DIM {
        rst += - (arr[i] * arr[i] / 2.0) as f64;
    }
    rst
}

impl<const N_DIM: usize> State<N_DIM> {
    pub fn new(seed: u64) -> Self {
    // Fill this
        let mut rng = SeedableRng::seed_from_u64(seed);
        let arr = [0.0; N_DIM];
        let mean ;
        let mut cov 
        let proposal_distribution = MultivariateNormal::new(mean, cov)
        Self { rng, arr, proposal_distribution }
    }

    pub fn take_step(&mut self) {
    // Fill this
    }
}
