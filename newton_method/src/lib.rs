#[allow(unused)]
pub fn newton_method(
    initial: f64,
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64,
    tolerance: usize            // Number of decimal places
) {
}

#[allow(unused)]
fn newton_iteration(
    initial: f64,
    f: fn(f64) -> f64,
    f_prime: fn(f64) -> f64
) {
    /*
        Step 1: Get initial x-value.
        Step 2: Calculate f and f_prime.
    */
}

#[cfg(test)]
mod tests {

   
}
