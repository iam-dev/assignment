// use lib::ChaumPedersen;
// use num_primes::{BigUint, Generator, Verification};
// use utils::calculate_q;
// use utils::generate_generator;
// use rand_chacha::ChaCha20Rng;
// use sha2::Sha256;
// use sigma_fun::{
//     secp256k1::{
//         self,
//         fun::{g, marker::*, Point, Scalar, G},
//     },
//     typenum::U16,
//     Either, FiatShamir, HashTranscript, Or,
// };
// use std::collections::HashMap;
// use std::env;
// use std::string::ToString;
// use utils::baby_step_giant_step;
// pub mod utils;

// fn print_help() {
//     println!("cargo run <order> <generator> <input>");
//     println!("- order:     the (prime) order n of the cyclic group");
//     println!("- generator: a generator for the group");
//     println!("- input:     the number we want the discrete logarithm of");
//     println!("Example: cargo run 31 3 6");
// }

fn main() {
    // 1) Setup steps
    // println!("====Begin setup steps====");
    // //Public parameters
    // // - 2 primes 𝑝,𝑞 such that 𝑝=2𝑞+1,
    // // and 2 elements 𝑔1,𝑔2∈ℤ∗𝑝 of order 𝑞 (i.e 𝑔1,𝑔2 are generators of a q-ordered sub-group of ℤ∗𝑝).
    // println!("Step 1.1) Generate P");
    // // Generate P (random large prime number)
    // let composite: BigUint = Generator::new_composite(10);
    // println!("Random prime number: {}", composite);

    // // Check if the number is a Composite Number
    // let is_composite: bool = Verification::is_composite(&composite);

    // // Asserts that 'is_composite' is true
    // assert_eq!(is_composite, true);

    // println!("Step 1.2) Calculate q, such that p = 2q + 1");
    // let q: BigUint = calculate_q(composite);
    // println!("q: {}", q);

    // println!("Step 1.3) generator g");
    // // println!("{}", modular_exponentiation_bn(2, 1, composite));
    // generate_generator(109);
    // println!("====End setup steps====");
    // let args: Vec<_> = env::args().collect();

    // if args.len() == 1 {
    //     print_help();
    //     return;
    // }

    // if args.len() != 4 {
    //     print_help();
    //     panic!("Wrong number of command line arguments");
    // }

    // let n: u32 = args[1].parse::<u32>().unwrap();
    // let alpha: u32 = args[2].parse::<u32>().unwrap();
    // let beta: u32 = args[3].parse::<u32>().unwrap();

    // println!("Discrete logarithm through baby-step giant-step algorithm.\n");
    // println!("Computing x such that {}^x mod {} = {}", alpha, n, beta);

    // match baby_step_giant_step(n, alpha, beta) {
    //     Result::Ok(value) => println!("Discrete logarithm is {}", value),
    //     Result::Err(_) => println!("Could not find discrete logarithm."),
    // }

    // // Pretend to choose H securely in a public setup
    // let H = Point::random(&mut rand::thread_rng());
    // println!("H = {}", H);

    // // our commitment will be to 1
    // let c = Scalar::from(1u32);
    // println!("c = {}", c);

    // // We use a 16-byte (128-bit) challenge length
    // type L = U16;

    // let (C, r) = {
    //     // make a pedersen commitment
    //     let r = Scalar::random(&mut rand::thread_rng());
    //     let C = g!(r * G + c * H)
    //         .normalize()
    //         .expect_nonzero("zero is computationally unreachable");
    //     (C, r)
    // };

    // println!("C = {}, r = {}", C, r);

    // // Our strategy is to prove that we know r such that either C = r * G or C - H = r * G using
    // // an OR composition between two standard knowledge of discrete logarithm proofs.
    // let statement = (
    //     C,
    //     g!(C - H)
    //         .normalize()
    //         .expect_nonzero("computationally unreachable"),
    // );
    // println!("statement = ({}, {})", statement.0, statement.1);

    // // since we are commiting to 1 we know the witness for the right hand side statement.
    // let witness = Either::Right(r);

    // // Our prover is going to prove knowledge of one of two point to the base G (either C or C - H).
    // type Protocol = Or<secp256k1::DLG<L>, secp256k1::DLG<L>>;

    // // Every protocol has an unambiguous name which is hashed into the transcript for protocol separation purposes.
    // assert_eq!(
    //     Protocol::default().to_string(),
    //     "or(DLG(secp256k1),DLG(secp256k1))"
    // );
    // // we want a non-interactive proof system so we apply the Fiat-Shamir transform with Sha256 as the challenge hash.
    // // We use ChaCha20Rng to help produce our announcement.
    // let proof_system = FiatShamir::<Protocol, HashTranscript<Sha256, ChaCha20Rng>>::default();

    // // Make the non-interactive proof -- pass in a system rng to make the proof more robust.
    // let proof = proof_system.prove(&witness, &statement, Some(&mut rand::thread_rng()));
    // // The verifier gets sent (C, proof)
    // {
    //     // The verifier's proof system doesn't need the rng
    //     let proof_system = FiatShamir::<Protocol, HashTranscript<Sha256>>::default();
    //     // They recreate the statement
    //     let statement = (C, g!(C - H).normalize().expect_nonzero("bogus commitment"));
    //     println!("statement = ({}, {})", statement.0, statement.1);
    //     // and verify it against the proof
    //     assert!(proof_system.verify(&statement, &proof));
    //     // The verifier is convinced of the statement and nothing else
    // }
}
