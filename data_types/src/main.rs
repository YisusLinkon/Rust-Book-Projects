fn main() {
    /*
    Integers type:
     - u8: length 8-bit ->  0 to 255
     - u16: length 16-bit
     - u32: length 32-bit
     - u64: length 64-bit
     - u128: length 128-bit
     - usize: length arch
     - i8  -> -128 to 127.
     - i16
     - i32  Default
     - i64
     - i128
     - isize

    Numbers literals:
    Decimal:  98_222 its same to 98222
    Hex: 0xff
    Octal: 0o77
    Binary: 0b1111_0000
    Byte (u8 only): b'A'

    Floating:
    f32
    f64 Default

    Boolean:
    true
    false

    Character:
    char

    Tuples: His size is unmutable
    tuple: (, , ,)

    */
    let tup = (5, 6, 7.7);

    let (_x, _y ,z) = tup;
    
    println!("Hello, world! {z}");
    let y = tup.1;
    println!("{y}");

    /*
    Arrays

    let array = [1; 7] create an array like [1, 1, 1, 1, ,1, ,1, 1]
    */
}
