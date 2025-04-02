// https://doc.rust-lang.org/book/ch20-06-macros.html

macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn create_vector() -> Vec<i32> {
    my_vec!(1, 2, 3, 4, 5)
}

macro_rules! declare_greeting {
    ($name:ident, $return_type:ty, $return_val:expr) => {
        impl $name {
            pub fn greet() -> $return_type {
                $return_val
            }
        }
    };
}


pub struct GreetingA;

pub struct GreetingB;

declare_greeting!(GreetingA, String, String::from("Hello from GreetingA!"));
declare_greeting!(GreetingB, u64, 0xFEFE);