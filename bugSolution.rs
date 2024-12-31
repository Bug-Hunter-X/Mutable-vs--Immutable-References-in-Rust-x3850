fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Output: x = 6

    // Correct way to modify x if an immutable reference already exists
    x += 1;
    println!("x = {}", x); //Output: x = 7
} 