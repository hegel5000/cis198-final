extern crate num;
extern crate libc;

#[macro_use]
pub mod vec_wrapper; 

pub mod vec_operations; 

pub mod rk4;

pub mod fft;

#[cfg(test)]
#[allow(unused_imports)]
mod tests_lib {
    use super::*;

    #[test]
    fn it_works() {
    }
}
