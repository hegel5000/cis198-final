use num::complex::{Complex, Complex64};
use libc::c_double;
use std::f64::consts::PI;

///
/// Compute the Discrete Fast Fourier Transform
/// of an f64 Vec with the Cooley-Turkey FFT algorithm
/// (implementation taken from Wikipedia)
///
pub fn fft(x: &Vec<Complex64>) -> &Vec<Complex64> {
    if x.len() == 1 {
        return x;
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
        let mut fft = Vec::<Complex64>::with_capacity(x.len());
        let k = 0;
        while k < x.len() {
            if k % 2 == 0 {
                fft.push(fft_1[k]);
            } else {
                fft.push(fft_2[k]);
            }
        }
        let k = 0;
        let n = x.len();
        while k < n/2 - 1 {
            let t = fft[k];
            let modulus: Complex64 = Complex::i() * 2.0 * PI * 
                (k as c_double) / (n as c_double);
            fft[k] = t + fft[k + n/2] * modulus.exp();
            fft[k + n/2] = t - fft[k + n/2] * modulus.exp();
        }
        x
    }
}

#[cfg(test)]
mod tests_fft {
    #[test]
    fn fft_compiles() {
    }
}
