// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

pub fn example_a() {
    let s1 = String::from("Hello");
    let _s2 = s1; // Ownership of the string is moved from s1 to s2
    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid
}

pub fn example_b() {

    fn concatenate(s1: String, s2: &String) -> String {
        s1 + s2
    }

    let s1 = "Hello".to_string();
    let s2 = "world!".to_owned();
    let s3 = concatenate(s1, &s2);
    println!("{s2} {s3}"); // This will work because s2 was borrowed as a reference rather than moved
    // println!("{s1}"); // This line would cause a compile-time error because s1 is no longer valid
}

#[allow(unused_mut, dead_code)]
pub fn example_c() {
    let mut s = String::from("Hello, Rust!");

    fn modify_string(s: &mut String) {
        s.push_str(" modified!");
    }
    
    // You cannot spawn both threads because 

    // let _handle1 = std::thread::spawn(move || { // move
    //     modify_string(&mut s);
    // });

    // let _handle2 = std::thread::spawn(move || {
    //     modify_string(&mut s);
    // });

    println!("{}", s);
}