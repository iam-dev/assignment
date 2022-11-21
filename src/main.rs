use ::std::sync::{Arc, Mutex};

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
}

impl TestMutex {
    fn new() -> Self {
        return TestMutex {
            users: Mutex::new(0),
        };
    }
}
