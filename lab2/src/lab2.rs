use std::convert::TryInto;

type U64Filter = std::iter::Filter<std::ops::RangeFrom<u64>, fn(&u64) -> bool>;

// ----------------------------------------------------------------------
// Part 1.
// ----------------------------------------------------------------------

pub fn divide_exact(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    match a % b {
        0 => Some(a / b),
        _ => None,
    }
}

pub fn checked_factorial(n: u64) -> Option<u64> {
    match n {
        0 => Some(1),
        n => Some(n.checked_mul(checked_factorial(n - 1)?)?),
    }
}

#[derive(Debug, PartialEq)]
pub enum GCDError {
    NotPositive,
    OutOfRange,
    NotAnInteger,
}

pub fn float_gcd(a: f32, b: f32) -> Result<f32, GCDError> {
    if a < 0.0 || b < 0.0 {
        return Err(GCDError::NotPositive);
    }
    if a > 16777215.0 || b > 16777215.0 {
        return Err(GCDError::OutOfRange);
    }
    if a.fract() != 0.0 || b.fract() != 0.0 {
        return Err(GCDError::NotAnInteger);
    }
    let mut m = a;
    let mut n = b;
    let mut t;
    while n != 0.0 {
        t = n;
        n = m % n;
        m = t
    }
    Ok(m)
}

pub fn divide_gcd(a: u16, b: u16) -> (u16, u16) {
    let gcd = float_gcd(a.into(), b.into()).unwrap() as i32;
    (
        divide_exact(a.into(), gcd).unwrap().try_into().unwrap(),
        divide_exact(b.into(), gcd).unwrap().try_into().unwrap(),
    )
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_divide_exact() {
        assert_eq!(divide_exact(1, 0), None);
        assert_eq!(divide_exact(1, 10), None);
        assert_eq!(divide_exact(100, 10), Some(10));
        assert_eq!(divide_exact(-100, 10), Some(-10));
        assert_eq!(divide_exact(44217, 153), Some(289));
    }

    #[test]
    fn test_checked_factorial() {
        assert_eq!(checked_factorial(0), Some(1));
        assert_eq!(checked_factorial(1), Some(1));
        assert_eq!(checked_factorial(5), Some(120));
        assert_eq!(checked_factorial(10), Some(3628800));
        assert_eq!(checked_factorial(21), None);
    }

    #[test]
    fn test_float_gcd() {
        assert_eq!(float_gcd(-1.0, 0.0), Err(GCDError::NotPositive));
        assert_eq!(float_gcd(0.0, -1.0), Err(GCDError::NotPositive));
        assert_eq!(float_gcd(2f32.powf(24.0), 1.0), Err(GCDError::OutOfRange));
        assert_eq!(float_gcd(1.0, 2f32.powf(24.0)), Err(GCDError::OutOfRange));
        assert_eq!(float_gcd(1.5, 2.0), Err(GCDError::NotAnInteger));
        assert_eq!(float_gcd(2.0, 1.5), Err(GCDError::NotAnInteger));
        assert_eq!(float_gcd(12.0, 32.0), Ok(4.0));
        assert_eq!(float_gcd(1479.0, 319.0), Ok(29.0));
    }

    #[test]
    fn test_divide_gcd() {
        assert_eq!(divide_gcd(5, 5), (1, 1));
        assert_eq!(divide_gcd(10, 5), (2, 1));
        assert_eq!(divide_gcd(31, 23), (31, 23));
        assert_eq!(divide_gcd(81, 9), (9, 1));
    }
}

// ----------------------------------------------------------------------
// Part 2.
// ----------------------------------------------------------------------

pub fn create_array_1() -> [i32; 10] {
    [0; 10]
}

pub fn create_array_2(a: i32, b: i32, c: i32, d: i32) -> [i32; 4] {
    [a, b, c, d]
}

pub fn create_vec_empty() -> Vec<i32> {
    Vec::new()
}

pub fn create_vec_macro_1() -> Vec<i32> {
    vec![0; 10]
}

pub fn create_vec_macro_2(a: i32, b: i32, c: i32, d: i32) -> Vec<i32> {
    vec![a, b, c, d]
}

pub fn add_two(x: &mut Vec<i32>) {
    x.push(2)
}

pub fn set_to_one(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = 1;
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_array_functions() {
        let mut arr1 = create_array_1();
        set_to_one(&mut arr1[..5]);
        assert_eq!(arr1, [1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);

        let arr2 = create_array_2(6, 7, 8, 9);
        assert_eq!(arr2, [6, 7, 8, 9]);
    }

    #[test]
    fn test_vec_functions() {
        let mut vec1 = create_vec_empty();
        add_two(&mut vec1);
        add_two(&mut vec1);
        add_two(&mut vec1);
        set_to_one(&mut vec1[..2]);
        assert_eq!(vec1, vec![1, 1, 2]);

        let mut vec2 = create_vec_macro_1();
        set_to_one(&mut vec2[5..]);
        assert_eq!(vec2, vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);

        let vec3 = create_vec_macro_2(6, 7, 8, 9);
        assert_eq!(vec3, vec![6, 7, 8, 9]);
    }
}

// ----------------------------------------------------------------------
// Part 3.
// ----------------------------------------------------------------------

pub fn vec_mean(data: &[f32]) -> f32 {
    data.iter().sum::<f32>() / data.len() as f32
}

pub fn vec_variance(data: &[f32]) -> f32 {
    let squares = data.iter().map(|x| x.powi(2)).collect::<Vec<f32>>();
    vec_mean(&squares) - vec_mean(data).powi(2)
}

pub fn iterator_factorial(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

pub fn primes_iterator() -> U64Filter {
    (2u64..).filter(|x| !(2u64..*x).any(|y| x % y == 0))
}

pub fn nth_prime(n: usize) -> u64 {
    primes_iterator().nth(n).unwrap()
}

pub fn n_primes(n: usize) -> Vec<u64> {
    primes_iterator().take(n).collect()
}

#[cfg(test)]
mod test3 {
    use super::*;

    #[test]
    fn test_iterator_factorial() {
        assert_eq!(iterator_factorial(0), 1);
        assert_eq!(iterator_factorial(1), 1);
        assert_eq!(iterator_factorial(3), 6);
        assert_eq!(iterator_factorial(5), 120);
        assert_eq!(iterator_factorial(10), 3628800);
    }

    #[test]
    fn test_n_primes() {
        assert_eq!(n_primes(1), vec![2]);
        assert_eq!(n_primes(2), vec![2, 3]);
        assert_eq!(n_primes(5), vec![2, 3, 5, 7, 11]);
        assert_eq!(n_primes(10), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(0), 2);
        assert_eq!(nth_prime(1), 3);
        assert_eq!(nth_prime(5), 13);
        assert_eq!(nth_prime(10), 31);
    }
}
