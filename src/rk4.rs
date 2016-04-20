use libc::c_double;

/*
 * Given a differential function dx(x, t),
 * initial condition x0,
 * and a list of times t,
 * find x(t) at each point in t
 */
#[no_mangle]
pub extern fn odeint(dx: (&Fn(c_double, c_double) -> c_double), 
                     x0: c_double, t_vec: Vec<c_double>) -> Vec<c_double> {
    // Need there to be at least two times for this method to work
    assert!(t_vec.len() >= 2);

    // Collect x values in this vector
    let mut result = Vec::<c_double>::new();
    result.push(x0);
    
    // Need to get step size by taking the difference between
    // two adjacent times
    for i in 0..(t_vec.len() - 1) { // Subtracting 1 from the length isn't a typo
        // This implementation is from Wikipedia
        let ti = t_vec[i];
        let tnext = t_vec[i+1];
        let h = tnext - ti;

        let xi = result.pop().unwrap();

        let k1 = dx(xi, ti);
        let k2 = dx(xi + h/2.0*k1, ti + h/2.0);
        let k3 = dx(xi + h/2.0*k2, ti + h/2.0);
        let k4 = dx(xi + h*k3, ti + h);

        let xnext = xi + h/6.0*(k1 + 2.0*k2 + 2.0*k3 + k4);

        result.push(xi);
        result.push(xnext);
    }
    result
}

#[cfg(test)]
mod tests_rk4 {
    use super::*;
    const THRESHOLD: f64 = 0.0000001;

    // Test differential to give to odeint
    #[allow(unused_variables)]
    fn velocity_one(x: f64, t: f64) -> f64 {
        1.0 // Velocity of particle is 1
    }

    #[allow(unused_variables)]
    fn free_fall(x: f64, t: f64) -> f64 {
        let g = -9.81;
        g*t
    }

    #[test]
    fn rk4_compiles() {
    }

    #[test]
    fn test_velocity_one() {

        let t = vec![0.0, 1.0];
        let x0 = 0.0;

        let mut result = odeint(&velocity_one, x0, t);

        assert!((result.pop().unwrap() - 1.0).abs() < THRESHOLD);
    }

    #[test]
    fn test_length() {
        let t = vec![0.0, 1.0, 2.0];
        let x0 = 0.0;

        let result = odeint(&velocity_one, x0, t);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_free_fall() {
        let mut times = Vec::<f64>::new();
        let mut i = 0.0;
        while i <= 10.0 {
            times.push(i);
            i += 0.1;
        }

        let x0 = 0.0;
        let mut result = odeint(&free_fall, x0, times);
        let expected_value = -490.5;

        assert!((result.pop().unwrap() - expected_value).abs() < THRESHOLD);
    }
}
