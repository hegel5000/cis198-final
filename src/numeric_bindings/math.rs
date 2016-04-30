use numeric::tensor::Tensor;

///In-place element-wise addition of the first argument.
#[no_mangle]
pub extern "C" fn add_64(x: Tensor<f64>, y: &Tensor<f64>) -> Tensor<f64> {
  x + y
}

///In-place dot-multiplication of the first argument.
#[no_mangle]
pub extern "C" fn dot_64(x: Tensor<f64>, y: &Tensor<f64>) -> Tensor<f64> {
  x.dot(y)
}
