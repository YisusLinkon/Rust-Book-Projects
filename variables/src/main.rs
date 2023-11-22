// const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;

fn main() {
    /*
    let mut x = 5;
    println!("This is your variable {x}");
    x = 6;
    println!("Now change the value to {x}");
    */

    let x = 5;
    println!("This is your variable {x}");
    let x = x + 1;
    println!("This is your variable {x}");
    {
        let x = x * 2;
        println!("This is your variable {x}");
    }

    println!("Finally this is your variable {x}");

    // Number of spaces
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Amount of spaces {spaces}");

    // With mutables we got an error because we change the type of variable
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // println!("Amount of spaces {spaces}");

}
