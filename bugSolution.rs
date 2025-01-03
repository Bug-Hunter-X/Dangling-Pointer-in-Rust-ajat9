fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements without dangling pointers
    for i in 0..vec.len() {
        println!("Value at index {}: {}", i, vec[i]);
    }

    // Alternatively, use iterators
    for val in &vec {
        println!("Value: {}", val);
    }
} 