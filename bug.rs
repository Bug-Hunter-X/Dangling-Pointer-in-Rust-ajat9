fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // ... some code that might modify or drop 'vec' ...

    // Dangling pointer!
    unsafe {
        println!("Value at ptr: {}", *ptr);
    }
}