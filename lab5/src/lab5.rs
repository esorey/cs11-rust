use std::fmt::Display;

// TODO: Write the `make_some` function.
pub fn make_some<T>(param: T) -> Option<T> {
    Some(param)
}

// TODO: Write the `make_pair` function.
pub fn make_pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

// TODO: Create and implement the `Value` struct
struct Value<T> {
    val: T,
}

impl<T> Value<T> {
    pub fn get(&self) -> &T {
        &self.val
    }
}

// TODO: Create and implement the `OneTwo` enum
enum OneTwo<A, B> {
    One(A),
    Two(A, B),
}

impl<A, B> OneTwo<A, B> {
    pub fn first(&self) -> &A {
        match self {
            OneTwo::One(a) => a,
            OneTwo::Two(a, _) => a,
        }
    }

    pub fn second(&self) -> Option<&B> {
        match self {
            OneTwo::One(_) => None,
            OneTwo::Two(_, b) => Some(b),
        }
    }
}

// TODO: Write the `clone` function.
pub fn clone<T: Clone>(x: &T) -> T {
    x.clone()
}

// TODO: Write the `cast` function.
pub fn cast<T, U>(t: T, u: U) -> U
where
    T: Into<U>,
{
    t.into()
}

// TODO: Write the `is_default` function.
pub fn is_default<T: Default + PartialEq>(t: T) -> bool {
    t == Default::default()
}

pub fn bounds_abound<X, Y, Z>(x: X, y: Y, z: Z)
where
    X: Copy + Display + PartialEq,
    Y: Copy + Display + Into<X> + PartialEq,
    Z: Copy + Display + Into<X> + Into<Y> + PartialEq,
{
    if x == y.into() {
        println!("{} and {} equal!", x, y);
    }

    if x == z.into() {
        println!("{} and {} equal!", x, z);
    }

    if y == z.into() {
        println!("{} and {} equal!", x, z);
    }
}

pub trait Name {
    fn name(&self) -> String;
}

pub struct Alpha {}

impl Name for Alpha {
    fn name(&self) -> String {
        String::from("Alpha")
    }
}

pub struct Beta {}

impl Name for Beta {
    fn name(&self) -> String {
        String::from("Beta")
    }
}

// TODO: Write the `name_vec_impl` function.
pub fn name_vec_impl(vec: Vec<impl Name>) -> Vec<String> {
    vec.iter().map(|x| x.name()).collect()
}

// TODO: Write the `name_vec_dyn` function.
pub fn name_vec_dyn(vec: Vec<Box<dyn Name>>) -> Vec<String> {
    vec.iter().map(|x| x.name()).collect()
}

pub fn demo_impl(n: usize) -> Vec<String> {
    let vec_alphas: Vec<Alpha> = (0..n).map(|_| Alpha {}).collect();
    let vec_betas: Vec<Beta> = (0..n).map(|_| Beta {}).collect();
    let mut names = name_vec_impl(vec_alphas);
    let mut beta_names = name_vec_impl(vec_betas);
    names.append(&mut beta_names);
    names
}

pub fn demo_dyn(n: usize) -> Vec<String> {
    // TODO: Fill in the `demo_dyn` function.
    let mut alt_vec: Vec<Box<dyn Name>> = vec![];
    for i in 0..n * 2 {
        alt_vec.push(if i % 2 == 0 {
            Box::new(Alpha {})
        } else {
            Box::new(Beta {})
        });
    }
    name_vec_dyn(alt_vec)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value() {
        let val1 = Value { val: 0.0f32 };
        assert_eq!(*val1.get(), 0.0f32);

        let val2 = Value {
            val: String::from("Hello"),
        };
        assert_eq!(*val2.get(), String::from("Hello"));
    }

    #[test]
    fn test_onetwo() {
        let mut onetwo: OneTwo<f32, u32> = OneTwo::One(0.0);
        assert_eq!(*onetwo.first(), 0.0f32);
        assert_eq!(onetwo.second(), None);

        onetwo = OneTwo::Two(1.0, 3);
        assert_eq!(*onetwo.first(), 1.0f32);
        assert_eq!(onetwo.second(), Some(&3));
    }

    #[test]
    fn test_is_default() {
        assert!(is_default(0.0f32));
        assert!(is_default(0i32));
        assert!(is_default(false));
    }

    #[test]
    fn test_demo_impl() {
        let vec_impl = demo_impl(4);
        for i in 0..4 {
            assert_eq!(vec_impl[i], String::from("Alpha"));
            assert_eq!(vec_impl[i + 4], String::from("Beta"));
        }
    }

    #[test]
    fn test_demo_dyn() {
        let vec_impl = demo_dyn(4);
        for i in 0..4 {
            assert_eq!(vec_impl[2 * i], String::from("Alpha"));
            assert_eq!(vec_impl[2 * i + 1], String::from("Beta"));
        }
    }
}
