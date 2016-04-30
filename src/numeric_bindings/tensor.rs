use libc::c_char;
use numeric::tensor::Tensor;
use std::cmp::min;
use std::ffi::CStr;
use std::ffi::CString;
use std::str::FromStr;

/// Maybe this could just be a function?  I'm not sure.
macro_rules! unsafe_take_cstr {
  ($literal:expr) => (unsafe { CStr::from_ptr($literal).to_string_lossy().into_owned() })
}

/// This is part of NumericRust's macro for making tensors.
/// 
/// This takes a string in the form of comma (',') delimited list of tokens 
/// which can be parsed as floating point numbers.
pub fn parse_row<T: FromStr + Copy>(literal: &str) -> Result<Tensor<T>, T::Err>
{ 
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
pub fn parse_rows<T: FromStr + Copy>(literal: &str) -> Result<Tensor<T>, T::Err>
{
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
    if min_width > 0 {
        min_width = min(width, min_width);
    } else {
        min_width = width;
    }
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

#[no_mangle]
pub extern "C" fn repr_tensor_64(tensor: Tensor<f64>) -> *const c_char {
  CString::new(format!("{}", tensor)).unwrap().as_ptr()
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let input_str = "7.0, 3.0, 2.0, -3.0, 2.0, -5.0";
        let result: Tensor<f64> = parse_row(&input_str).unwrap();
        let expected = Tensor::new(vec![7.0, 3.0, 2.0, -3.0, 2.0, -5.0]);
        assert_eq!(result.data(), expected.data());
    }

    #[test]
    fn test_parse_rows_one_row() {
        let input_str = "7.0, 3.0, 2.0, -3.0, 2.0, -5.0";
        let result: Tensor<f64> = parse_rows(&input_str).unwrap();
        println!("{:?}", *result.data());
        let expected = Tensor::new(vec![7.0, 3.0, 2.0, -3.0, 2.0, -5.0]);
        assert_eq!(result.data(), expected.data());
    }
    
    #[test]
    fn test_parse_rows() {
        let input_str = "7.0, 3.0, 2.0; -3.0, 2.0, -5.0";
        let result: Tensor<f64> = parse_rows(&input_str).unwrap();
        println!("{:?}", *result.data());
        let expected = Tensor::new(vec![7.0, 3.0, 2.0, -3.0, 2.0, -5.0])
            .reshape(&[2, 3]);
        assert_eq!(result.data(), expected.data());
    }
}
