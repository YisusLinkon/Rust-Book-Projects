use std::io;

fn main() {
    println!("Now choose what do u want to do:");
    println!("1. Convert celsius from fahrenheit");
    println!("2. Convert fharenheit from celsius");
    println!("3. Calculate fibonnaci n");
    println!("Another option to exit.");

    loop {
        println!("Please enter a number: ");
        let mut opt: String = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Fail to read line");

        let opt: u32 = opt.trim().parse().expect("Please type a number!");

        if opt == 1 {
            println!("Please enter a number of fahrenheit grades: ");
            let mut grades: String = String::new();
            io::stdin().read_line(&mut grades).expect("Fail to read");
            let grades: i32 = match grades.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("The grades are: {}", conver_to_celsius_from_fahrenheit(grades));
        } else if opt == 2 {
            println!("Please enter a number of celsius grades: ");
            let mut grades: String = String::new();
            io::stdin().read_line(&mut grades).expect("Fail to read");
            let grades: i32 = match grades.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("The grades are: {}", conver_to_fahrenheit_from_celsius(grades));
        } else if opt == 3 {
            println!("Please enter a number of fibonnaci: ");
            let mut number: String = String::new();
            io::stdin().read_line(&mut number).expect("Fail to read");
            let number: u8 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("The fibonnaci is: {}", fibonacci(number));
        } else {
            break;
        }

        println!("\n\nNow choose what do u want to do:");
        println!("1. Convert celsius from fahrenheit");
        println!("2. Convert fharenheit from celsius");
        println!("3. Calculate fibonnaci n");
        println!("Another option to exit.");
    }
}

fn fibonacci(n: u8) -> usize {
    let mut a: usize = 1;
    let mut b: usize = 1;
    if n == 1 {
        a
    } else if n == 2 {
        b
    } else {
        let mut c: usize = a + b;
        let mut count: u8 = 3;
        while count < n {
            a = b;
            b = c;
            count += 1;
            c = a + b;
        }
        c
    }
}

fn conver_to_celsius_from_fahrenheit(temp: i32) -> i32 {
    (temp - 32) * (5/9)
}

fn conver_to_fahrenheit_from_celsius(temp: i32) -> i32 {
    (temp * 9/5) + 32
}
