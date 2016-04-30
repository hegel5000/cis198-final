use libc::c_char;
use std::cmp::min;
use std::ffi::CStr;
use std::ffi::CString;
use std::ops::Deref;
use std::str::FromStr;

macro_rules! unsafe_alloc_vec_f64 { ($e:expr) => { {
  //use std::mem::forget;
  use std::mem::transmute;
  let b: Box<Vec<f64>> = Box::new($e);
  let raw: *mut Vec<f64> = unsafe { transmute(b) };
  //forget(b);
  raw
} } }

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
  unsafe_alloc_vec_f64!(vec!(1.0, 2.0))
}

///Exportable version of parse_row.
#[no_mangle]
//pub extern "C" fn parse_vec_64(literal: *const c_char) -> Option<Vec<f64>> {
pub extern "C" fn parse_vec_64(literal: *const c_char) -> *mut Vec<f64> {
  let lit_from_c = unsafe { CStr::from_ptr(literal).to_string_lossy().into_owned() };
  unsafe_alloc_vec_f64!(parse_vec(&lit_from_c).unwrap())
}

#[no_mangle]
pub extern "C" fn print_vec_64(vec_c: *mut Vec<f64>) {
  unsafe { println!("{}", repr_vec(&*vec_c)) };
}

#[no_mangle]
pub extern "C" fn repr_vec_64(vec_c: *mut Vec<f64>) -> *const c_char {
  unsafe { CString::new(repr_vec(&*vec_c)).unwrap().as_ptr() }
}

pub fn repr_vec(vec: &Vec<f64>) -> String {
  let mut buf = String::new();
  let mut i = vec.into_iter();
  match i.next() {
    Some(first) => { buf = buf + &first.to_string(); },
    None => { (); },
  }
  for x in i {
    buf = buf + "," + &x.to_string();
  }
  buf
}

#[cfg(test)]
mod tests_rk4 {
  use super::*;
  use std::ffi::CStr;
  use std::ffi::CString;
  use std::str::from_utf8;

  #[test]
  fn test_repr_vec_64() {
    let pair = one_two();
    let pair_repr = unsafe { CStr::from_ptr(repr_vec_64(pair)).to_bytes() };
    let pair_repr_str: &str = from_utf8(pair_repr).unwrap();
    assert_eq!(pair_repr_str, "1,2");
  }

  #[test]
  fn test_parse_vec_64() {
    let vec = parse_vec_64(CString::new("1.5,2.2,42,0,99.9999,-32.4,-2.0").unwrap().as_ptr());
    let vec_repr = unsafe { CStr::from_ptr(repr_vec_64(vec)).to_bytes() };
    let vec_repr_str = from_utf8(vec_repr).unwrap();
    assert_eq!(vec_repr_str, "1.5,2.2,42,0,99.9999,-32.4,-2");
  }
}
