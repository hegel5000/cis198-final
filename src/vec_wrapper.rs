use libc::c_char;
use std::cmp::min;
use std::ffi::CStr;
use std::ffi::CString;
use std::ops::Deref;
use std::str::FromStr;

/// This is _meant_ to be `Vec<f64> -> *mut Vec<f64>.  More importantly, any 
/// memory allocated with this will leak, though hopefully it will end up up
/// deallocated by Python or whatever scripting language this goes to?
/// TODO: write an exportable destructor
macro_rules! unsafe_alloc_vec_f64 { ($e:expr) => { {
  //use std::mem::forget;
  use std::mem::transmute;
  let b: Box<Vec<f64>> = Box::new($e);
  let raw: *mut Vec<f64> = unsafe { transmute(b) };
  //forget(b);
  raw
} } }

/// Since there's so much more stuff out there to help with FFIing (that doesn't
/// even grammatically make sense) C style strings, but nothing really to help
/// with doing so with other sorts of C style arrays, I decided it would be
/// easier only to send strings back and forth.
///
/// # Examples
///
/// ```
/// assert_eq!(parse_vec("4.0,1.0,1.3").unwrap(), vec!(4.0, 1.0, 1.3))
/// ```
pub fn parse_vec<T>(literal: &str) -> Result<Vec<T>, T::Err>
    where T: FromStr + Copy {
  let mut out = Vec::new();
  for tok in literal.split(",") {
    out.push(try!(T::from_str(tok.trim())));
  }
  Ok(out)
}

/// This is mostly here for testing and debugging purposes on the Python side.
#[no_mangle]
pub extern "C" fn one_two() -> *mut Vec<f64> {
  unsafe_alloc_vec_f64!(vec!(1.0, 2.0))
}

/// FFI version of `parse_vec`.
#[no_mangle]
pub extern "C" fn parse_vec_64(literal: *const c_char) -> *mut Vec<f64> {
  let lit_from_c = unsafe { CStr::from_ptr(literal).to_string_lossy().into_owned() };
  unsafe_alloc_vec_f64!(parse_vec(&lit_from_c).unwrap())
}

/// I couldn't figure out how to get Python to deal with the arrays on the other
/// side of *const c_chars sent from Rust, so I just gave up and added this T_T.
/// See scirust_examples.py.
#[no_mangle]
pub extern "C" fn print_vec_64(vec_c: *mut Vec<f64>) {
  unsafe { println!("{}", repr_vec(&*vec_c)) };
}

/// FFI version of `repr_vec`.
#[no_mangle]
pub extern "C" fn repr_vec_64(vec_c: *mut Vec<f64>) -> *const c_char {
  unsafe { CString::new(repr_vec(&*vec_c)).unwrap().as_ptr() }
}

/// This is essentially the inverse of `parse_vec`.
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
