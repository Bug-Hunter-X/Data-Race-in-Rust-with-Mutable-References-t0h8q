fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x

    *y = 10; // Modify x through y
    *z = 20; // Modify x through z  This is where the error occurs if not handled correctly.
}