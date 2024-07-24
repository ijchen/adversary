pub mod generators;
mod input_generator;
mod report;
mod test_runner;

pub use rand;

pub use generators::{any, Canonical};
pub use input_generator::InputGenerator;
pub use report::Report;
pub use test_runner::{run_test, run_test_panics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        run_test(|v: &bool| !v, any(), &mut crate::rand::thread_rng()).unwrap_err();
        run_test(|v: &bool| *v, any(), &mut crate::rand::thread_rng()).unwrap_err();
    }
}
