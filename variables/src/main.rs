fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let tup: (i32, f64, bool) = (2, 6.5, false);
    let (x, y, z) = tup; // note shadowing
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // likewise can access with .idx operator on tup
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("The value of tup.0 is: {tup0}");
    println!("The value of tup.1 is: {tup1}");
    println!("The value of tup.2 is: {tup2}");
}
