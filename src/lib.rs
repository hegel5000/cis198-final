extern crate libc;

pub mod rk4;
#[macro_use]
pub mod vec_wrapper; 
pub mod vec_operations; 

#[cfg(test)]
#[allow(unused_imports)]
mod tests_lib {
    use super::*;

    #[test]
    fn it_works() {
    }
}
