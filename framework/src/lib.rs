use std::panic::{RefUnwindSafe, catch_unwind};

pub fn runner(tests: &[&Testable]){
    for t in tests {
        print!("{}... ", t.name());
        match catch_unwind(|| t.run()) {
            Err(_) => println!("Failed"),
            Ok(_) => println!("Passed")
        }
    }
}


pub trait Testable:  RefUnwindSafe {
    fn name(&self) -> String;
    fn run(&self);
}