fn main() {
    println!("Hello, world!");
    another_function(5);
}

fn another_function(x: i32) {
    println!("This is our number: {x}");

    let new = {
        let y = 3;
        y + 1
    };
    println!("This is our number: {new}");
    // println!("This is our number: {five()}"); No works!
    let new = five();
    println!("This is our number: {new}");
}

fn five() -> u32 {
    5
}