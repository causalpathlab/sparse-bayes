//! Bayesian sparse regression.
//!
//! - [`sgvb`]: stochastic-gradient variational Bayes with local
//!   reparameterization, over SuSiE, multilevel SuSiE, bi-SuSiE, spike-and-slab
//!   and Gaussian variational families, with Gaussian, Poisson, negative
//!   binomial, RSS, vMF and weighted likelihoods; CAVI SuSiE.
//! - [`mcmc`]: an elliptical-slice MCMC engine (single and batched chains,
//!   diagnostics) and sparse regression on it.

pub mod mcmc;
pub mod sgvb;
