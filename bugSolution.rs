fn main() {
    let mut x = 5;
    // Use a mutex to protect access to x
    let x_mutex = std::sync::Mutex::new(x);

    // Create a mutable reference to x with locking to protect against data race
    let mut y = x_mutex.lock().unwrap();
    *y = 10;

    // Create another mutable reference using locking
    let mut z = x_mutex.lock().unwrap();
    *z = 20; 
    println!("x: {}", *z); // Now the operation is thread safe
}