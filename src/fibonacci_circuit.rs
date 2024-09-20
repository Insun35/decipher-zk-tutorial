use plonky2::{
    field::extension::Extendable, hash::hash_types::RichField, iop::target::Target,
    plonk::circuit_builder::CircuitBuilder,
};

use crate::{fibonacci::MAX_N, utils::less_than_or_equal_to};

pub struct FibonacciTargets {
    pub first: Target,
    pub second: Target,
}

pub fn fibonacci_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    n: usize,
    nth_fibonacci: u64,
) -> FibonacciTargets {
    let ttrue = builder._true();

    let first = builder.add_virtual_target();
    let second = builder.add_virtual_target();
    let nth_fibonacci = builder.constant(F::from_canonical_u64(nth_fibonacci));

    // assert!(n <= MAX_N);
    let n_target = builder.constant(F::from_canonical_usize(n));
    let max_n = builder.constant(F::from_canonical_usize(MAX_N));
    let is_le_than_max = less_than_or_equal_to(builder, n_target, max_n, 32);
    builder.connect(is_le_than_max.target, ttrue.target);

    let mut prev1 = first;
    let mut prev2 = second;
    let mut current = builder.zero();
    for _ in 2..=n {
        current = builder.add(prev1, prev2);

        prev1 = prev2;
        prev2 = current;
    }

    builder.connect(nth_fibonacci, current);

    builder.register_public_input(n_target);
    builder.register_public_input(current);

    FibonacciTargets { first, second }
}
