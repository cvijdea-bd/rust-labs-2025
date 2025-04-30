pub struct StillEasyTupleButWithStructKeyword(pub i32, pub f64, pub char);

pub struct UselessStruct;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            age: 0,
        }
    }

    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I'm {} years old.",
            self.name, self.age
        );
    }
}
