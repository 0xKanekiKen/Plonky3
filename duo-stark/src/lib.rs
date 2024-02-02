//! A minimal univariate STARK framework.

#![no_std]

extern crate alloc;

mod proof;
mod prover;
mod verifier;

pub use proof::*;
pub use prover::*;
pub use verifier::*;
