use plonky2::{
    field::extension::Extendable, hash::hash_types::RichField, iop::target::Target,
    plonk::circuit_builder::CircuitBuilder,
};

use crate::{
    fibonacci::MAX_N,
    utils::{greater_than, less_than_or_equal_to},
};

pub struct Witness {
    pub first: Target,
    pub second: Target,
}

pub fn fibonacci_circuit<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    n: usize,
    nth_fibonacci: u64,
) -> Witness {
    let ttrue = builder._true();
    let one = builder.one();

    let first = builder.add_virtual_target();
    let second = builder.add_virtual_target();
    let nth_fibonacci = builder.constant(F::from_canonical_u64(nth_fibonacci));

    // assert!(n > 1 && n <= MAX_N);
    let n_target = builder.constant(F::from_canonical_usize(n));
    let max_n = builder.constant(F::from_canonical_usize(MAX_N));
    let is_greater_than_one = greater_than(builder, n_target, one, 32);
    let is_le_than_max = less_than_or_equal_to(builder, n_target, max_n, 32);
    let range_check = builder.and(is_greater_than_one, is_le_than_max);
    builder.connect(range_check.target, ttrue.target);

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

    Witness { first, second }
}
