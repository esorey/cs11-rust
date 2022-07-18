use std::collections::HashMap;

macro_rules! print_expression {
    ($x:expr) => {
        println!("{}", $x)
    };
}

macro_rules! make_function {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("Created a function!")
        }
    };
    ($fn_name:ident, $body:expr) => {
        fn $fn_name() {
            println!("{}", $body)
        }
    };
}

make_function!(test1);
make_function!(test2, 42);

macro_rules! display_from_debug {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
    };
}

#[derive(Debug)]
enum TestEnum {
    A,
    B,
}

display_from_debug!(TestEnum);

macro_rules! dict {
    ( $( $k:literal: $v:literal),* ) => {
        {
            let mut temp = HashMap::new();
            $(
                temp.insert($k, $v);
            )*
            temp
        }
    };
}

fn main() {
    print_expression!(1 + 2);

    test1();
    test2();

    println!("{}, {}", TestEnum::A, TestEnum::B);

    let x = dict!(1: 2, 2: 3, 3: 4);
    println!("{:?}", x);
}
