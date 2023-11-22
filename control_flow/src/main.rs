fn main() {
    let number: u8 = 3;

    if number < 5 {
        println!("Hello, world!");
    } else {
        println!("Hello, math!");
    }

    let number: u8 = 5;
    if number < 5 {
        println!("Hello, world!");
    } else {
        println!("Hello, math!");
    }

    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 7 };
    println!("This is the number: {number}");

    loops();
    complex_loop();
    while_over_array();
    for_over_array();
    count_with_for();
}


fn loops() {
    /* We have 3 types of loops, loop, while, for */
    let mut counter: u8 = 0;
    let result: u8 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("This is the results: {result}");
}

fn complex_loop() {
    let mut count: u8 = 0;
    'counting_up: loop {
        println!("Count: {count}");
        let mut remain: u8 = 10;

        loop {
            println!("Remaing: {remain}");
            if remain == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remain -= 1;
        }
        count += 1;
    }

    println!("Final count: {count}");
}

fn while_over_array() {
    let a: [u8; 4] = [10, 20, 30, 40];
    let mut index: usize = 0;

    while index < 4 {
        println!("Value: {}", a[index]);
        index += 1;
    }
}

fn for_over_array() {
    let a: [u8; 4] = [10, 20, 30, 40];

    for element in a {
        println!("Value with for: {}", element);
    }
}

fn count_with_for() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    for number in (1..4) {
        println!("{number}");
    }
}