use libc::c_char;
use numeric::tensor::Tensor;
use std::cmp::min;
use std::ffi::CStr;
use std::str::FromStr;

/// Maybe this could just be a function?  I'm not sure.
macro_rules! unsafe_take_cstr {
  ($literal:expr) => (unsafe { CStr::from_ptr($literal).to_string_lossy().into_owned() })
}

/// This is part of NumericRust's macro for making tensors.
/// 
/// This takes a string in the form of comma (',') delimited list of tokens 
/// which can be parsed as floating point numbers.
pub fn parse_row<T>(literal: &str) -> Result<Tensor<T>, T::Err>
  where T: FromStr + Copy { 
  let mut out = Vec::new();
  for tok in literal.split(",") {
    out.push(try!(T::from_str(tok.trim())));
  }
  Ok(Tensor::new(out))
}

/// This is the full functionality of NumericRust's macro for making tensors.
/// I felt like just turning into a function which can be called on the Python
/// side would be the easiest way to make tensor structs; the alternative would
/// probably involve a function which takes in an array of pointers to arrays
/// of doubles, or something, which would be harder to marshall than a single
/// string with delimeters.
/// 
/// Each row in the output `Tensor` works the same way as it does in `parse_row`,
/// except that rows of the tensor can be delimited by semicolons.
/// 
/// # Examples
///
/// ```
/// parse_columns(&"7.0, 3.0, 2.0; -3.0, 2.0, -5.0")
/// ```
///
/// This returns out a 2x3 matrix: 
/// ```
/// [ 7.0  3.0 -3.0 ]
/// [-3.0  2.0 -5.0 ]
/// ```
pub fn parse_rows<T>(literal: &str) -> Result<Tensor<T>, T::Err>
  where T: FromStr + Copy {
  let mut out = Vec::new();
  let mut height = 0;
  let mut min_width = 0;
  for line in literal.split(";") {
    height = height + 1;
    let mut width = 0;
    for tok in line.split(",") {
      out.push(try!(T::from_str(tok.trim())));
      width = width + 1;
    }
    min_width = min(width, min_width);
  }
  Ok(Tensor::new(out).reshape(&[min_width as isize, height as isize]))
}

///Exportable version of parse_row.  Possibly not necessary?
#[no_mangle]
pub extern "C" fn parse_tensor_1_64(literal: *const c_char) -> Option<Tensor<f64>> {
  let lit_from_c = unsafe_take_cstr!(literal);
  parse_row(&lit_from_c).ok()
}

///Exportable version of parse_rows.
#[no_mangle]
pub extern "C" fn parse_tensor_2_64(literal: *const c_char) -> Option<Tensor<f64>> {
  let lit_from_c = unsafe_take_cstr!(literal);
  parse_rows(&lit_from_c).ok()
}
