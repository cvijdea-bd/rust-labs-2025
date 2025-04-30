pub fn implicit_lifetime(s: &str) -> &str {
    s
}

pub fn explicit_lifetime<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

struct Foo<'a> {
    a: &'a str,
}

impl Foo<'_> {
    #[allow(dead_code)]
    fn foo(&self) {
        println!("{}", self.a);
    }
}

pub fn example_invalid() {
    let s = String::from("a");
    let f = Foo { a: &s };
    // let handle = std::thread::spawn(move || {
    //     f.foo();
    // });
    println!("{}", f.a);
}
