use num_primes::{BigUint, Generator, Verification};
use num_traits::cast::ToPrimitive;
// use num_traits::Zero;
// use std::collections::HashMap;
// use std::convert::TryInto;

// Generate a random prime number of size bits
// Parameters: size: size of the prime number in bits
// Returns: a random prime number in BigUint format
pub fn generate_prime(size: usize) -> Result<BigUint, String> {
    // Generate  Prime of size bits
    let prime: BigUint = Generator::new_prime(size);

    // Check if the number is a prime number
    if Verification::is_prime(&prime) {
        return Ok(prime);
    } else {
        return Err("Error: The number is not a prime number".to_string());
    }
}

// Calculate q is a prime number, such that p = 2q + 1
// Parameters: p: a prime number
// Returns: q: (p - 1) / 2
pub fn calculate_q(p: &BigUint) -> Result<BigUint, String> {
    let one: BigUint = BigUint::new(vec![1]);
    let two: BigUint = BigUint::new(vec![2]);
    let q: BigUint = (p - one) / two;

    // Check if the number is a Composite Number
    if Verification::is_composite(&q) {
        return Ok(q);
    } else {
        return Err("Error: The number is not a composite number".to_string());
    }
}

// Generate random number
// Parameters: size: size of the prime number in bits
// Returns: random number in BigUint format
pub fn generate_random(size: usize) -> Result<BigUint, String> {
    // Generate 2 randoms numbers a and b
    let a: BigUint = Generator::new_composite(size);

    if Verification::is_composite(&a) {
        return Ok(a);
    } else {
        return Err("Error: The number is not a composite number".to_string());
    }
}

// Generate the generators g, which is of the order of q and is in the group 𝑍∗𝑝.
// g <--- ∈𝑍∗𝑞
// Theorem: ɑ∈ℤp* is a generator of ℤp* if
//          ɑ(p-1/q)   ≇ 1 mod p
//          for all primes q such that (q / p -1)
pub fn generators_g(p: &BigUint) -> Vec<BigUint> {
    let mut result: Vec<BigUint> = Vec::new();
    let one: BigUint = BigUint::new(vec![1]);
    let two: BigUint = BigUint::new(vec![2]);

    let p_min_one: BigUint = p - one;
    let exp: BigUint = &p_min_one / two;
    println!("p_min_one {}", p_min_one.clone());
    println!("exp {}", exp);

    for i in 2..100 {
        let j: BigUint = BigUint::new(vec![i]);
        let modulo: BigUint = j.modpow(&exp, &p_min_one);
        println!("modulo {}", modulo);

        // test if modulo is not congruent to 1 mod p
        let convert_module: u128 = modulo.to_u128().unwrap();
        if convert_module > 1 {
            result.push(BigUint::new(vec![i]));
        }

        // //Performance measure: if we find a generator, we can stop the loop
        if result.len() == 5 {
            break;
        }
    }
    return result;
}
