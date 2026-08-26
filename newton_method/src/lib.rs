#[allow(unused)]
pub fn newton_method(
    initial: f64,
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64,
    tolerance: usize            // Number of significant digits
) {
}

#[allow(unused)]
fn newton_iteration(
    initial: f64,
    function: fn(f64) -> f64,
    derivative: fn(f64) -> f64
) -> f64 
{
    /*  
        Step 3: Define the line tangent to f that intersects (initial, f(initial)).
        Step 4: Find the x-intercept of that line
        Step 5: Decide if we're done or need to iterate again.
    */

    let f: f64 = function(initial);
    let f_prime: f64 = derivative(initial);
    /*
        From y = f'(x_0)(x-x_0) + f(x_0), since we care about the x-intercept,
        we can set y = 0.
        Thus, f'(x_0)(x-x_0) + f(x_0) = 0 becomes
        x = (x_0 * f'(x_0) - f(x_0)) / f'(x_0)
    */
    (initial * derivative(initial) - function(initial)) / derivative(initial)
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_example() {
        let initial: f64 = 2.0;
        let x_1: f64 = newton_iteration(initial, _quadratic_function, _quadratic_function_derivative);
        assert!(0.0 < x_1 && x_1 < initial);
    }
}

fn _quadratic_function(x: f64) -> f64 {
    x.powi(2)
}

fn _quadratic_function_derivative(x: f64) -> f64 {
2.0 * x
}
   



