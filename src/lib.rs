extern crate libc;
extern crate numeric;

pub mod rk4;

pub mod numeric_bindings {
  pub mod math;
  pub mod tensor;
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests_lib {
    use super::*;

    #[test]
    fn it_works() {
    }
}
