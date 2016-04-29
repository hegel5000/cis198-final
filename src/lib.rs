extern crate libc;

pub mod rk4;
pub mod vec_operations;
pub mod vec_wrapper; 

#[cfg(test)]
#[allow(unused_imports)]
mod tests_lib {
    use super::*;

    #[test]
    fn it_works() {
    }
}
