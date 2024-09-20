use plonky2::{
    field::extension::Extendable,
    hash::hash_types::RichField,
    iop::target::{BoolTarget, Target},
    plonk::circuit_builder::CircuitBuilder,
};

pub fn range_check_optimized<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    target: Target,
    n: usize,
) {
    if let Some(value) = builder.target_as_constant(target) {
        assert!(F::to_canonical_u64(&value) < (1u64 << n))
    } else {
        builder.range_check(target, n)
    }
}

/// Returns the bits of the given number. Will panic if `n >= F::BITS`
pub fn num_to_bits<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    n: usize,
    x: Target,
) -> Vec<BoolTarget> {
    if n < F::BITS {
        // safe to use `split_le`
        return builder.split_le(x, n);
    }
    // ToDo: handle the conversion if `n == F::BITS` and `F` is Goldilocks field

    panic!("cannot call this method with n > F::BITS");
}

/// Returns true if a < b in the first n bits, False otherwise.
/// Will panic if `n >= F::BITS-1`
pub fn less_than<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    // enforce that a < 2^n and b < 2^n
    range_check_optimized(builder, a, n);
    range_check_optimized(builder, b, n);
    less_than_unsafe(builder, a, b, n)
}

/// Returns true if a < b in the first n bits, False otherwise.
/// Will panic if `n >= F::BITS-1`.
/// This variant is unsafe since it assumes that `a < 2^n` and `b < 2^n`;
/// undefined behavior may occur if this assumption is not ensured by the
/// caller
pub fn less_than_unsafe<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    assert!(n < F::BITS - 1);

    let power_of_two = builder.constant(F::from_canonical_u64(1 << n));

    let mut lin_pol = builder.add(a, power_of_two);
    // 2^n + a - b
    lin_pol = builder.sub(lin_pol, b);

    let binary = num_to_bits(builder, n + 1, lin_pol);
    // bin(2^n + a - b)[n] == false is correct only when a < b otherwise
    // 2^n + a - b > 2^n so binary[n] will be set
    builder.not(binary[n])
}

/// Returns true if a > b in the first n bits. False otherwise.
/// Will panic if `n >= F::BITS-1`
pub fn greater_than<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    less_than(builder, b, a, n)
}

/// Returns true if a > b in the first n bits, False otherwise.
/// Will panic if `n >= F::BITS-1`.
/// This variant is unsafe since it assumes that `a < 2^n` and `b < 2^n`;
/// undefined behavior may occur if this assumption is not ensured by the
/// caller
pub fn greater_than_unsafe<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    less_than_unsafe(builder, b, a, n)
}

/// Returns true if a <= b in the first n bits. False otherwise.
/// Will panic if `n >= F::BITS-1`
pub fn less_than_or_equal_to<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    // enforce that a < 2^n and b < 2^n
    range_check_optimized(builder, a, n);
    range_check_optimized(builder, b, n);
    let one = builder.one();
    let b_plus_1 = builder.add(b, one);
    less_than_unsafe(builder, a, b_plus_1, n)
}

/// Returns true if a <= b in the first n bits, False otherwise.
/// Will panic if `n >= F::BITS-1`.
/// This variant is unsafe since it assumes that `a < 2^n` and `b < 2^n`;
/// undefined behavior may occur if this assumption is not ensured by the
/// caller
pub fn less_than_or_equal_to_unsafe<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    let one = builder.one();
    let b_plus_1 = builder.add(b, one);
    less_than_unsafe(builder, a, b_plus_1, n)
}
/// Returns true if a >= b in the first n bits. False otherwise.
/// Will panic if `n >= F::BITS-1`
pub fn greater_than_or_equal_to<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    // enforce that a < 2^n and b < 2^n
    range_check_optimized(builder, a, n);
    range_check_optimized(builder, b, n);
    let one = builder.one();
    let a_plus_1 = builder.add(a, one);
    less_than(builder, b, a_plus_1, n)
}

/// Returns true if a >= b in the first n bits, False otherwise.
/// Will panic if `n >= F::BITS-1`.
/// This variant is unsafe since it assumes that `a < 2^n` and `b < 2^n`;
/// undefined behavior may occur if this assumption is not ensured by the
/// caller
pub fn greater_than_or_equal_to_unsafe<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    a: Target,
    b: Target,
    n: usize,
) -> BoolTarget {
    let one = builder.one();
    let a_plus_1 = builder.add(a, one);
    less_than_unsafe(builder, b, a_plus_1, n)
}
