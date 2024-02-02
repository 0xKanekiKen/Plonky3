//! A minimal univariate STARK framework.

#![no_std]

extern crate alloc;

mod config;
mod decompose;
mod folder;
mod proof;
mod prover;
pub mod symbolic_builder;
mod symbolic_expression;
mod symbolic_variable;
mod verifier;
mod zerofier_coset;

#[cfg(debug_assertions)]
pub mod check_constraints;

pub use config::*;
pub use decompose::*;
pub use folder::*;
pub use proof::*;
pub use prover::*;
pub use verifier::*;
pub use zerofier_coset::*;

#[cfg(debug_assertions)]
pub use check_constraints::*;
