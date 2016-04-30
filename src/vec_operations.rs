/// In-place element-wise addition.
/// Truncates to length of shorter vector.
#[no_mangle]
pub extern "C" fn pairwise_add_64(x: &Vec<f64>, y: &Vec<f64>) -> *mut Vec<f64> {
  let out = unsafe { pairwise_add(&*x, &*y) };
  unsafe_alloc_vec_f64!(out)
}

pub fn pairwise_add(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
  let mut out = Vec::new();
  for (x, y) in x.into_iter().zip(y) {
    out.push(x+y);
  }
  out
}

/// In-place dot-multiplication of the first argument.
#[no_mangle]
pub extern "C" fn dot_64(x: *mut Vec<f64>, y: *mut Vec<f64>) -> f64 {
  unsafe { dot(&*x, &*y) }
}

pub fn dot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
  let mut out = 0.0;
  for (x, y) in x.into_iter().zip(y) {
    out = out + x*y;
  }
  out
}
