use framework::Testable;
pub use format_macro::my_test;

pub struct SimpleFnTest {
    pub name: &'static str,
    pub fnc: fn()
}

impl Testable for SimpleFnTest {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn run(&self) {
        (self.fnc)()
    }
}

