use libc::c_char;
use std::cmp::min;
use std::ffi::CStr;
use std::ffi::CString;
use std::str::FromStr;

/// Maybe this could just be a function?  I'm not sure.
macro_rules! unsafe_take_cstr {
  ($literal:expr) => ()
}

/// This is part of NumericRust's macro for making tensors.
/// 
/// This takes a string in the form of comma (',') delimited list of tokens 
/// which can be parsed as floating point numbers.
pub fn parse_vec<T>(literal: &str) -> Result<Vec<T>, T::Err>
    where T: FromStr + Copy {
  let mut out = Vec::new();
  for tok in literal.split(",") {
    out.push(try!(T::from_str(tok.trim())));
  }
  Ok(out)
}

#[no_mangle]
pub extern "C" fn one_two() -> *mut Vec<f64> {
  &mut vec!(1.0, 2.0)
}

///Exportable version of parse_row.
#[no_mangle]
//pub extern "C" fn parse_vec_64(literal: *const c_char) -> Option<Vec<f64>> {
pub extern "C" fn parse_vec_64(literal: *const c_char) -> *mut Vec<f64> {
  let lit_from_c = unsafe { CStr::from_ptr(literal).to_string_lossy().into_owned() };
  &mut parse_vec(&lit_from_c).unwrap()
}

#[no_mangle]
pub extern "C" fn repr_vec_64(tensor: *const Vec<f64>) -> *const c_char {
  CString::new(format!("{:?}", tensor)).unwrap().as_ptr()
}
