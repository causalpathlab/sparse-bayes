# sparse-bayes

Bayesian sparse regression in Rust on [candle](https://github.com/huggingface/candle).

- `sgvb`: stochastic-gradient variational Bayes with the local
  reparameterization trick: SuSiE, multilevel SuSiE, bi-SuSiE, spike-and-slab
  and Gaussian variational families; Gaussian, Poisson, negative binomial, RSS,
  von Mises-Fisher and weighted likelihoods; CAVI SuSiE.
- `mcmc`: an elliptical-slice MCMC engine (single and batched chains,
  diagnostics) and sparse regression on it.

Matrix utilities come from [legume-numeric](https://crates.io/crates/legume-numeric).
