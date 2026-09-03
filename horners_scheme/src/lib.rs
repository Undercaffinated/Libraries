use std::ops::Add;
use std::ops::Mul;
use std::fmt::Display;

/// # Horner's Algorithm
/// 
/// Goal: Evaluate polynomials in linear time.
/// 
/// Approach: Suppose you have a polynomial p in the form ∑_(i = 0)^n(a_i * x^i) where a_i are the coefficients.
/// We observe such expressions can be rewritten as
/// a0 + x(a1 + x(a2 ...)).
/// 
/// To define this function recursively, we evaluate the most deeply nested elements first, then work outward.
/// 
/// This approach allows for a polynomial to be solved in n additions and multiplications.
/// 
/// Important Note: The coefficients vector expects the coefficients to be organized in the order a0...a_n.

pub fn horners_evaluate<T: Add<Output = T> + Mul<Output = T> + Display + Clone>(x: T, coefficients: Vec<T>) -> T {
    // This is not robust. May need to fix in the future.
    assert_ne!(coefficients.len(), 0);

    // If there is only one element, then the polynomial is a constanct since a0 * x^0 = a0.
    if coefficients.len() == 1 { return coefficients.last().unwrap().clone(); }
    
    let mut acc = coefficients.last().unwrap().clone();
    println!("{}", acc);
    // Instead of reversing the list, let's just iterate backwards.
    for i in (1..coefficients.len()).rev() {
        acc = x.clone() * acc + coefficients[i-1].clone();
        println!("{}", acc);
    }
    acc
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //p(X) = 2*x^2 - 7x + 5, p(6) = 72 - 42 + 5 = 35
        let polynomial: Vec<i32> = Vec::from([5, -7, 2]);
        assert_eq!(horners_evaluate(6_i32, polynomial), 35);

        // p(x) = 3x^3 - 4x^2 + 0x - 17 = 81 - 36 - 17 = 
        let polynomial: Vec<i32> = Vec::from([-17, 0, -4, 3]);
        let solution: i32 = 28;
        assert_eq!(horners_evaluate(3, polynomial), solution);
    }
}
