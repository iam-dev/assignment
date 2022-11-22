use lib::{calculate_q, generators_g};
use num_primes::{BigUint, Generator};
use num_traits::Pow;

pub mod lib;

fn main() {
    // SERVER SIDE
    println!("===========SERVER SIDE===========");
    println!("1) Setup steps");
    println!("Step 1.1) Generate Prime number p");
    // We take p = 23 as an example
    let p: BigUint = BigUint::from(23u32);
    println!("p: {}", p);

    println!("Step 1.2) Compute q, such that p = 2q + 1");
    let q: BigUint = calculate_q(&p).unwrap();
    println!("q: {}", q);

    // We take b = 8 as an example
    println!("Step 1.3) Generate random b");
    let b: BigUint = BigUint::from(8u32);
    println!("b: {}", b);

    println!("Step 1.4) Generate generators, such that g is a generator of Zp*");
    let generators: Vec<BigUint> = generators_g(&p);
    println!("generators: {:?}", generators);

    // Let's choose generator g = 4
    // g[2] = 4
    let g: BigUint = generators[2].clone();

    println!("Step 1.5) Compute h");
    // h = g^b mod p
    let h: BigUint = g.modpow(&b, &p);
    println!("h = g^b mod p => {}", h);

    println!("2) Register process");
    println!("===========CLIENT SIDE===========");
    // Choose secret x
    println!("Step 2.1) Choose secret number x");
    let x: BigUint = BigUint::from(6u32);
    println!("x: {}", x);

    println!("Step 2.2) Compute y1 and y2");

    //y1 = g^x mod p
    let y1: BigUint = g.modpow(&x, &p);
    println!("y1 = {}^{} mod {} = {}", g, x, p, y1);

    // y2 = h^x mod p
    let y2: BigUint = h.modpow(&x, &p);
    println!("y2 = {}^{} mod {} = {}", h, x, p, y2);
    println!("===========SEND y1 and y2 to SERVER===========");
    println!("===========use Register gRPC function===========");

    println!("===========CLIENT SIDE===========");
    println!("3) Login process");
    println!("Step 3.1) Random k");
    //Lets choose k = 7
    let k: BigUint = BigUint::from(7u32);
    println!("k: {}", k);

    println!("Step 3.2) Compute r1 and r2");
    //r1 = g^k mod p
    let r1: BigUint = g.modpow(&k, &p);
    println!("r1 = {}", r1);

    // r2 = h^k mod p
    let r2: BigUint = h.modpow(&k, &p);
    println!("r2 = {}", r2);
    println!("===========SEND r1 and r2 to SERVER===========");
    println!("===========use CreateAuthenticationChallenge gRPC function===========");

    println!("===========SERVER SIDE===========");
    println!("===========CreateAuthenticationChallenge function===========");
    println!("Step 3.3) Generate random c");
    //Lets choose c = 4
    let c: BigUint = BigUint::from(300u32);
    println!("c: {}", c);
    println!("===========SEND c to CLIENT===========");
    println!("===========as response on CreateAuthenticationChallenge function===========");

    println!("===========CLIENT SIDE===========");
    println!("Step 3.4) Compute s");
    // s = k - c * x mod q
    let s: BigUint = &k - &c * &x % &q;
    println!("s = {}", s);
    println!("===========SEND s to SERVER===========");
    println!("===========use VerifyAuthentication gRPC function===========");

    println!("===========SERVER SIDE===========");
    println!("===========VerifyAuthentication function===========");
    println!("Step 3.5) verify");

    // 1) verify g^s * y1^c  mod p == r1
    let verify_one: BigUint = g.pow(&s) * y1.pow(&c) % &p;
    println!("verify_one: {}", verify_one);
    assert_eq!(r1, verify_one);

    // 2) verify h^s * y1^c  mod p == r2
    let verify_two: BigUint = h.pow(&s) * y2.pow(&c) % &p;
    println!("verify_two: {}", verify_two);
    assert_eq!(r2, verify_two);
}
