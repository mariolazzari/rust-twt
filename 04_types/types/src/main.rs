fn main() {
    // Scalar types
    println!("Scalar types");

    println!("Integers:");
    let x: i8 = -8;
    println!("Signed 8bit integer: {}", x);

    let x: i16 = -16;
    println!("Signed 16bit integer: {}", x);

    let x: i32 = -32;
    println!("Signed 32bit integer: {}", x);

    let x: i64 = -64;
    println!("Signed 64bit integer: {}", x);

    let x: i128 = -128;
    println!("Signed 128bit integer: {}", x);

    let x: u8 = 8;
    println!("Unsigned 8bit integer: {}", x);

    let x: u16 = 16;
    println!("Unsigned 16bit integer: {}", x);

    let x: u32 = 32;
    println!("Unsigned 32bit integer: {}", x);

    let x: u64 = 64;
    println!("Unsigned 64bit integer: {}", x);

    let x: u128 = 128;
    println!("Unsigned 128bit integer: {}", x);

    println!("\nFloats");

    let x: f32 = 123213.12313;
    println!("Single precision: {}", x);

    let x: f64 = 123213.12313;
    println!("Double precision: {}", x);

    println!("Booleans");

    let x: bool = true;
    println!("Boolean: {}", x);

    let x: bool = false;
    println!("Boolean: {}", x);

    println!("Chars (use signle quote)");

    let x: char = 'M';
    println!("Char: {}", x);

    // Compound types
    println!("\nCompound types");

    println!("\nTuples");
    let tup: (i32, bool, char) = (1, true, 'M');
    println!("Tuple 1st element: {}", tup.0);
    println!("Tuple 2nd element: {}", tup.1);
    println!("Tuple 3rd element: {}", tup.2);

    println!("\nMutable Tuples");
    let mut tup: (i32, bool, char) = (1, true, 'M');
    tup.0 = 11;
    tup.1 = false;
    tup.2 = 'L';

    println!("Tuple 1st element: {}", tup.0);
    println!("Tuple 2nd element: {}", tup.1);
    println!("Tuple 3rd element: {}", tup.2);

    println!("\nArrays");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array 1st: {}", arr[0]);

    println!("\nMutable Arrays");
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array 1st: {}", arr[0]);
    arr[0] = -1;
    println!("Array 1st: {}", arr[0]);
    // arrays must be initialized: arr = [] -> error!

    println!("\nTypes example:");
    let a: u8 = 8;
    let b = a; // implicity u8: u32 -> error"
    println!("a = {}, B = {}", a, b);
}
