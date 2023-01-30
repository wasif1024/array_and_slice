use std::mem;
fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    println!("{}",xs[0]);
    println!("{}",ys[0]);
    println!("number of elements in array: {}", xs.len());
    println!("number of elements in array: {}", mem::size_of_val(&xs));
    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
