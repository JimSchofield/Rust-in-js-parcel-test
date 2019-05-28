#[no_mangle]
pub fn add(a: i32, b: i32) -> i32
    // works if imported in js
    return a + b;
}

#[no_mangle]
pub fn do_something() -> &'static str {
    let s = "Hi there!";
    return s;
}
