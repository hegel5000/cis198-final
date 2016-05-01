/// FFI version of `pairwise_add`.
#[no_mangle]
pub extern "C" fn pairwise_add_64(x: &Vec<f64>, y: &Vec<f64>) -> *mut Vec<f64> {
  let out = pairwise_add(&*x, &*y);
  unsafe { unsafe_alloc_vec_f64!(out) }
}

/// Adds each element of the same index from from each `Vec`.
/// The output `Vec` is the length of the shorter of the two inputs.
pub fn pairwise_add(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
  let mut out = Vec::new();
  for (x, y) in x.into_iter().zip(y) {
    out.push(x+y);
  }
  out
}

/// FFI version of `dot`.
#[no_mangle]
pub extern "C" fn dot_64(x: *mut Vec<f64>, y: *mut Vec<f64>) -> f64 {
  unsafe { dot(&*x, &*y) }
}

/// Multiplies each pair of numbers at the same index in the two input `Vec`s,
/// and returns the sum of those products.  Ignores additional elements if one
/// input `Vec` is longer than the other.
pub fn dot(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
  let mut out = 0.0;
  for (x, y) in x.into_iter().zip(y) {
    out = out + x*y;
  }
  out
}
