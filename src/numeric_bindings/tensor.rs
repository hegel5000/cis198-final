use libc::c_char;
use numeric::tensor::Tensor;
use std::cmp::min;
use std::ffi::CStr;
use std::str::FromStr;

macro_rules! unsafe_take_cstr {
  ($literal:expr) => (unsafe { CStr::from_ptr($literal).to_string_lossy().into_owned() })
}

pub fn parse_row<T>(literal: &str) -> Result<Tensor<T>, T::Err>
  where T: FromStr + Copy { 
  let mut out = Vec::new();
  for tok in literal.split(",") {
    out.push(try!(T::from_str(tok)));
  }
  Ok(Tensor::new(out))
}

pub fn parse_columns<T>(literal: &str) -> Result<Tensor<T>, T::Err>
  where T: FromStr + Copy {
  let mut out = Vec::new();
  let mut height = 0;
  let mut min_width = 0;
  for line in literal.split(";") {
    height = height + 1;
    let mut width = 0;
    for tok in line.split(",") {
      out.push(try!(T::from_str(tok)));
      width = width + 1;
    }
    min_width = min(width, min_width);
  }
  Ok(Tensor::new(out).reshape(&[min_width as isize, height as isize]))
}

#[no_mangle]
pub extern "C" fn parse_tensor_1_64(literal: *const c_char) -> Option<Tensor<f64>> {
  let lit_from_c = unsafe_take_cstr!(literal);
  parse_row(&lit_from_c).ok()
}

#[no_mangle]
pub extern "C" fn parse_tensor_2_64(literal: *const c_char) -> Option<Tensor<f64>> {
  let lit_from_c = unsafe_take_cstr!(literal);
  parse_columns(&lit_from_c).ok()
}
