fn main() {
    /*
        let m1: String = String::from("Hello");
        let m2: String = String::from("World");
        /*
        It doesn't works because the values was moved into greet function.
            greet(m1, m2);
            println!("{} {}", m1, m2); 
        */
        let (m1_copy, m2_copy): (String, String) = greet(m1, m2);
        let s: String = format!("{} {}", m1_copy, m2_copy);
        println!("{}", s);
    */

    // But its better use references to this.
    let word1: String = String::from("Hola");
    let word2: String = String::from("Mundo");
    greet(&word1, &word2);
    let whole: String = format!("{} {}", word1, word2);
    println!("{}", whole);


    //Now we will use dereference
    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x;
    *x += 1;

    let r1: &Box<i32> = &x;
    let _b: i32 = **r1;

    let r2: &i32 = &*x;
    let _c: i32 = *r2;

    // Another example of dereferences

    let x: Box<i32> = Box::new(-1);
    let x_abs: i32 = i32::abs(*x); // Explicit dereference
    let x_abs2: i32 = x.abs(); // Implicit dereference
    assert_eq!(x_abs, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs: i32 = i32::abs(**r); // Explicit dereference
    let r_abs2: i32 = r.abs(); // Implicit deference
    assert_eq!(r_abs, r_abs2);

    let s: String = String::from("Hello");
    let s_len: usize = str::len(&s);
    let s_len2: usize = s.len();
    assert_eq!(s_len, s_len2);
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
    //(g1, g2)
}