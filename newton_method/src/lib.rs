#[allow(unused)]
pub fn newton_method(
    initial: f64,
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64,
    tolerance: usize, // Number of significant digits
) {
}

#[allow(unused)]
fn newton_iteration(x0: f64, f: fn(f64) -> f64, dfdx: fn(f64) -> f64) -> f64 {
    let f_x0: f64 = f(x0);
    let dfdx_x0: f64 = dfdx(x0);
    /*
        Point-Slope of a Line := y-f(x0) = m(x-x0)

        Since we want to define this line to be parallel to the tangent line at (x0, f(x0)), we define
        m∈ℝ = f'(x0), resulting in y-f(x0) = f'(x0)*(x-x0).

        We add f(x0) to both sides, resulting in y = f'(x0)*(x-x0) + f(x0)

        From this, since we care about the x-intercept we can set y = 0.
        Thus, f'(x0)*(x-x0) + f(x0) = 0.

        Solving for x, we get x = (x0 * f'(x0) - f(x_0)) / f'(x_0)
    */
    (x0 * dfdx_x0 - f_x0) / dfdx_x0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_example() {
        let initial: f64 = 2.0;
        let x_1: f64 =
            newton_iteration(initial, _quadratic_function, _quadratic_function_derivative);
        // Here we note 0.0 is the actual x-intercept. For the expression f(x) = x^2,
        // newton_method(n -> inf, x^2, 2x) == 0.0
        assert!(0.0 < x_1 && x_1 < initial);
    }
}

fn _quadratic_function(x: f64) -> f64 {
    x.powi(2)
}

fn _quadratic_function_derivative(x: f64) -> f64 {
    2.0 * x
}
