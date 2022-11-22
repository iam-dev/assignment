use ::std::sync::{Arc, Mutex};
use num_primes::BigUint;

#[derive(Debug)]
pub struct TestMutex {
    users: Mutex<i32>,
}

fn main() {
    let bla = TestMutex::new();
    {
        let mut num = bla.users.lock().unwrap();
        *num = 5;
    }

    println!("bla= {:?}", bla);

    let calc: BigUint = BigUint::from(4u32);
    println!(
        "calc= {:?}",
        calc.modpow(&BigUint::from(10u32), &BigUint::from(10009u32))
    );

    let g: i32 = 4;
    let y1: i32 = 3131;
    let calc2: i32 = (g.pow(300) * y1.pow(300)) % 10009;
    println!("calc2= {:?}", calc2);
}

impl TestMutex {
    fn new() -> Self {
        return TestMutex {
            users: Mutex::new(0),
        };
    }
}
