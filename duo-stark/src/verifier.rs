use alloc::vec::Vec;

use p3_air::Air;

use crate::Proof;
use p3_uni_stark::symbolic_builder::{get_log_quotient_degree, SymbolicAirBuilder};
use p3_uni_stark::{StarkConfig, VerifierConstraintFolder};

pub fn verify<SC, A>(
    _config: &SC,
    air: &[&A],
    _challenger: &mut SC::Challenger,
    _proof: &Proof<SC>,
) -> Result<(), VerificationError>
where
    SC: StarkConfig,
    A: Air<SymbolicAirBuilder<SC::Val>> + for<'a> Air<VerifierConstraintFolder<'a, SC::Challenge>>,
{
    let log_quotient_degrees = air
        .iter()
        .map(|air| get_log_quotient_degree::<SC::Val, A>(air))
        .collect::<Vec<_>>();
    let _quotient_degrees = log_quotient_degrees
        .iter()
        .map(|&log_degree| 1 << log_degree)
        .collect::<Vec<_>>();

    // let Proof {
    //     commitments,
    //     opened_values,
    //     opening_proof,
    //     degree_bits,
    // } = proof;

    // let air_widths = air.iter().map(|var| <A as BaseAir<SC::Val>>::width(var)).collect::<Vec<_>>();
    // let quotient_chunks = quotient_degrees.iter().map(|&degree| degree * <SC::Challenge as AbstractExtensionField<SC::Val>>::D).collect::<Vec<_>>();
    // assert!(air_widths.len() == quotient_chunks.len(), "air and quotient chunks must have the same length");
    // assert!(air_widths.len() == opened_values.trace_local.len(), "air and trace local must have the same length");
    // assert!(air_widths.len() == opened_values.trace_next.len(), "air and trace next must have the same length");

    // let valid_shape: bool = air_widths.iter().zip_eq(quotient_chunks.iter()).zip_eq(opened_values.trace_local.iter()).zip_eq(opened_values.trace_next.iter()).all(|(air_width, quotient_chunk, trace_local, trace_next)| {
    //     air_width == &trace_local.len() && air_width == &trace_next.len() && air_width == &quotient_chunk.len()
    // });

    // let valid_shape = opened_values.trace_local.len() == air_width
    //     && opened_values.trace_next.len() == air_width
    //     && opened_values.quotient_chunks.len() == quotient_chunks;
    // if !valid_shape {
    //     return Err(VerificationError::InvalidProofShape);
    // }

    // let g_subgroups = degree_bits.iter().map(|&degree_bits| SC::Val::two_adic_generator(degree_bits)).collect::<Vec<_>>();

    // challenger.observe(commitments.trace.clone());
    // let alpha: SC::Challenge = challenger.sample_ext_element();
    // challenger.observe(commitments.quotient_chunks.clone());
    // let zeta: SC::Challenge = challenger.sample_ext_element();

    // let local_and_next = [vec![zeta, zeta * g_subgroup]];
    // let commits_and_points = &[
    //     (commitments.trace.clone(), local_and_next.as_slice()),
    //     (
    //         commitments.quotient_chunks.clone(),
    //         &[vec![zeta.exp_power_of_2(log_quotient_degree)]],
    //     ),
    // ];
    // let values = vec![
    //     vec![vec![
    //         opened_values.trace_local.clone(),
    //         opened_values.trace_next.clone(),
    //     ]],
    //     vec![vec![opened_values.quotient_chunks.clone()]],
    // ];
    // let dims = &[
    //     vec![Dimensions {
    //         width: air_width,
    //         height: 1 << degree_bits,
    //     }],
    //     vec![Dimensions {
    //         width: quotient_chunks,
    //         height: 1 << degree_bits,
    //     }],
    // ];
    // config
    //     .pcs()
    //     .verify_multi_batches(commits_and_points, dims, values, opening_proof, challenger)
    //     .map_err(|_| VerificationError::InvalidOpeningArgument)?;

    // // Derive the opening of the quotient polynomial, which was split into degree n chunks, then
    // // flattened into D base field polynomials. We first undo the flattening.
    // let challenge_ext_degree = <SC::Challenge as AbstractExtensionField<SC::Val>>::D;
    // let mut quotient_parts: Vec<SC::Challenge> = opened_values
    //     .quotient_chunks
    //     .chunks(challenge_ext_degree)
    //     .map(|chunk| {
    //         chunk
    //             .iter()
    //             .enumerate()
    //             .map(|(i, &c)| <SC::Challenge as AbstractExtensionField<SC::Val>>::monomial(i) * c)
    //             .sum()
    //     })
    //     .collect();
    // // Then we reconstruct the larger quotient polynomial from its degree-n parts.
    // reverse_slice_index_bits(&mut quotient_parts);
    // let quotient: SC::Challenge = zeta
    //     .powers()
    //     .zip(quotient_parts)
    //     .map(|(weight, part)| part * weight)
    //     .sum();

    // let z_h = zeta.exp_power_of_2(*degree_bits) - SC::Challenge::one();
    // let is_first_row = z_h / (zeta - SC::Val::one());
    // let is_last_row = z_h / (zeta - g_subgroup.inverse());
    // let is_transition = zeta - g_subgroup.inverse();
    // let mut folder = VerifierConstraintFolder {
    //     main: TwoRowMatrixView {
    //         local: &opened_values.trace_local,
    //         next: &opened_values.trace_next,
    //     },
    //     is_first_row,
    //     is_last_row,
    //     is_transition,
    //     alpha,
    //     accumulator: SC::Challenge::zero(),
    // };
    // air.eval(&mut folder);
    // let folded_constraints = folder.accumulator;

    // // Finally, check that
    // //     folded_constraints(zeta) = Z_H(zeta) * quotient(zeta)
    // if folded_constraints != z_h * quotient {
    //     return Err(VerificationError::OodEvaluationMismatch);
    // }

    Ok(())
}

#[derive(Debug)]
pub enum VerificationError {
    InvalidProofShape,
    /// An error occurred while verifying the claimed openings.
    InvalidOpeningArgument,
    /// Out-of-domain evaluation mismatch, i.e. `constraints(zeta)` did not match
    /// `quotient(zeta) Z_H(zeta)`.
    OodEvaluationMismatch,
}
