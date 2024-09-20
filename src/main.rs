use anyhow::Result;
use decipher_zk_tutorial::{
    fibonacci::{fibonacci, MAX_N},
    fibonacci_circuit::fibonacci_circuit,
};
use plonky2::field::types::Field;
use plonky2::iop::witness::PartialWitness;
use plonky2::iop::witness::WitnessWrite;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::{
    circuit_builder::CircuitBuilder,
    circuit_data::CircuitConfig,
    config::{GenericConfig, PoseidonGoldilocksConfig},
};
use rand::{thread_rng, Rng};
use std::fs;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

fn prove(
    first: u32,
    second: u32,
    n: usize,
    nth_fibonacci: u64,
) -> Result<ProofWithPublicInputs<F, C, D>> {
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

    let witness = fibonacci_circuit(&mut builder, n, nth_fibonacci);

    let mut pw = PartialWitness::new();
    pw.set_target(witness.first, F::from_canonical_u32(first));
    pw.set_target(witness.second, F::from_canonical_u32(second));

    let data = builder.build::<C>();
    data.prove(pw)
}

struct PublicInputs {
    n: usize,
    nth_fibonacci: u64,
}

fn generate_random_prover() -> PublicInputs {
    let mut rng = thread_rng();

    // Random inputs
    let first = rng.gen();
    let second = rng.gen();
    let n = rng.gen_range(0..MAX_N);

    // Compute actual nth_fibonacci from first and second
    let nth_fibonacci = fibonacci(first, second, n);

    // Proof of a statement of the form:
    // "I know the first and second elements that construct the given nth element of the Fibonacci sequence"
    let proof = prove(first, second, n, nth_fibonacci).unwrap();
    println!(
        "Prover: Generate proof that {} and {} construct {}th Fibonacci number {}",
        first, second, proof.public_inputs[0], proof.public_inputs[1]
    );

    let proof_serialized = serde_json::to_string(&proof).unwrap();
    fs::write("target/proof.json", proof_serialized).expect("Unable to write file");

    PublicInputs { n, nth_fibonacci }
}

fn main() -> Result<()> {
    let public_input = generate_random_prover();

    let proof: ProofWithPublicInputs<F, C, D> = serde_json::from_str(
        &fs::read_to_string("target/proof.json").expect("Unable to read file"),
    )
    .expect("Unable to deserialize proof");

    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());

    fibonacci_circuit(&mut builder, public_input.n, public_input.nth_fibonacci);

    let data = builder.build::<C>();

    println!(
        "Verifier: Verify that {} is valid {}th Fibonacci number",
        public_input.nth_fibonacci, public_input.n
    );
    data.verify(proof)
}
