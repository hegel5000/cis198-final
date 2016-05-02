use num::complex::{Complex, Complex64};
use libc::c_double;
use std::f64::consts::PI;

///
/// Compute the Discrete Fast Fourier Transform
/// of an f64 Vec with the Cooley-Turkey FFT algorithm
/// (implementation taken from Wikipedia)
///
pub fn fft(x: &Vec<Complex64>) -> Vec<Complex64> {
    let mut res: Vec<Complex64> = Vec::<Complex64>::with_capacity(x.len());
    if x.len() == 1 {
        res.push(x[0]);
        return res
    }
    else {
        // Follow the Cooley-Tukey recursive implementation
        let x_1: &Vec<Complex64> = &x.iter()
            .enumerate()
            .filter(|&(index, _)| (index as u32) % 2 == 0)
            .map(|(_, elt)| *elt)
            .collect();
        let x_2: &Vec<Complex64> = &x.iter()
            .enumerate()
            .filter(|&(index, _)| (index as u32) % 2 == 1)
            .map(|(_, elt)| *elt)
            .collect();
        let fft_1 = fft(x_1);
        let fft_2 = fft(x_2);
        let mut k = 0;
        while k < x.len() {
            if k % 2 == 0 {
                res.push(fft_1[k/2]);
            } else {
                res.push(fft_2[k/2]);
            }
            k += 1;
        }
        let mut k = 0;
        let n = x.len();
        while k < n/2 - 1 {
            let t = res[k];
            let modulus: Complex64 = Complex::i() * 2.0 * PI * 
                (((k as u32) / (n as u32)) as c_double);
            res[k] = t + res[k + n/2] * modulus.exp();
            res[k + n/2] = t - res[k + n/2] * modulus.exp();
            k += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests_fft {
    use super::*;
    use num::complex::Complex64;

    #[test]
    fn fft_compiles() {
    }

    #[test]
    fn view_fft() {
        let x: Vec<Complex64> = vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()
            .map(|x| Complex64::new(*x, 0.0)).collect();
        println!("{:?}", fft(&x));
    }
}
