pub enum EasyEnum {
    Variant1,
    Variant2,
    Variant3,
}

pub enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
}

#[repr(u32)]
pub enum FreakyEnum {
    Empty,
    VariantA = 0xFF0000,
    VariantB(String, u32, bool),
    VariantC {
        name: String,
        age: u32,
        is_student: bool,
    },
}

// Useful builtin enums

// Option<T> - represents an optional value

// Result<T, E> - represents a value that can be either Ok or Err

pub fn option_match() {
    let x: Option<i32> = Some(5);

    let x = Option::Some(5);
    // let y = Option::None;

    match x {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    if x.is_some() {
        println!("Value exists");
    } else {
        println!("No value");
    }

    if x.is_none() {
        println!("No value");
    } else {
        println!("Value exists");
    }

    if let Some(value) = x {
        println!("Value: {}", value)
    }
}

pub fn result_match() {
    let x: Result<i32, String> = Ok(5);

    match x {
        Ok(value) => println!("Value: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // if x.is_ok() {
    //     println!("Value exists");
    // } else {
    //     println!("Error");
    // }

    // if x.is_err() {
    //     println!("Error");
    // } else {
    //     println!("Value exists");
    // }
}
