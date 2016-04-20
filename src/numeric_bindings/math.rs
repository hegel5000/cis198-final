use numeric::math;
use numeric::tensor::Tensor;

#[no_mangle]
pub extern "C" fn log_64(x: Tensor<f64>, base: f64) -> Tensor<f64> {
  math::log(x, base)
}
