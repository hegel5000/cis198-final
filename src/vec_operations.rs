macro_rules! unsafe_alloc_vec_f64 { ($e:expr) => { {
  use std::mem::transmute;
  let b: Box<Vec<f64>> = Box::new($e);
  let raw: *mut Vec<f64> = unsafe { transmute(b) };
  raw
} } }

/// In-place element-wise addition.
/// Truncates to length of shorter vector.
#[no_mangle]
pub extern "C" fn add_64(x: &Vec<f64>, y: &Vec<f64>) -> *mut Vec<f64> {
  let mut out = Vec::new();
  for (x, y) in x.into_iter().zip(y) {
    out.push(x+y);
  }
  unsafe_alloc_vec_f64!(out)
}

///In-place dot-multiplication of the first argument.
#[no_mangle]
pub extern "C" fn dot_64(x: Vec<f64>, y: &Vec<f64>) -> f64 {
  let mut out = 0.0;
  for (x, y) in x.into_iter().zip(y) {
    out = out + x*y
  }
  out
}
