use more_asserts as ma;
use num_primes::{BigUint, Generator, Verification};
use num_traits::cast::ToPrimitive;

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

    return Ok(q);
}

// Generate random number
// Parameters: size: size of the prime number in bits
// Returns: random number in BigUint format
pub fn generate_random(size: usize) -> Result<BigUint, String> {
    // Generate 2 randoms numbers a and b
    let a: BigUint = Generator::new_composite(size);
    return Ok(a);
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

    for i in 2..10000 {
        let j: BigUint = BigUint::new(vec![i]);
        let modulo: BigUint = j.modpow(&exp, &p_min_one);
        println!("modulo {}", modulo);

        // test if modulo is not congruent to 1 mod p
        let convert_module: u128 = modulo.to_u128().unwrap();
        if convert_module != 1 {
            result.push(BigUint::new(vec![i]));
        }

        // //Performance measure: if we find a generator, we can stop the loop
        if result.len() == 5 {
            break;
        }
    }
    return result;
}

fn main() {}

#[cfg(test)]
#[test]
fn generate_prime_test() {
    let _x: BigUint = generate_prime(512).unwrap();
    assert!(Verification::is_prime(&_x));
}

#[cfg(test)]
#[test]
fn calculate_q_test() {
    // q: (p - 1) / 2
    let p: BigUint = BigUint::from(23u32);
    let expected: BigUint = BigUint::from(11u32);
    let q: BigUint = calculate_q(&p).unwrap();
    assert_eq!(q, expected);
}

#[cfg(test)]
#[test]
fn generate_random_test() {
    let _x: BigUint = generate_random(512).unwrap();
}

#[cfg(test)]
#[test]
fn generators_g_test() {
    let two: BigUint = BigUint::from(2u32);
    let four: BigUint = BigUint::from(4u32);
    let six: BigUint = BigUint::from(6u32);
    let eight: BigUint = BigUint::from(8u32);
    let ten: BigUint = BigUint::from(10u32);
    let p: BigUint = BigUint::from(17u32);
    let generators: Vec<BigUint> = generators_g(&p);
    assert_eq!(generators[0], two);
    assert_eq!(generators[1], four);
    assert_eq!(generators[2], six);
    assert_eq!(generators[3], eight);
    assert_eq!(generators[4], ten);
    ma::assert_le!(generators.len(), 5);
}
