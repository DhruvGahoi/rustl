fn main() {
    //{
    //      let s = "hello"; // we cannot mut this string
    //}

    // let s = String::from("hello"); // this will be stored in heap
    // appending string
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literal to string
                            // drop(s);
    println!("{s}");

    let mut x = 5;
    let y = x;
    x = 10;
    println!("X is {x} and Y is {y}");

    // let s = String::from("I am X");
    // let y = s;
    // println!("s is {s}");
    //                ^^^ value borrowed here after move
    //
    //Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    let s1 = String::from("Hello!");
    let _s2 = s1.clone();
    // The conclusion is Rust doesn't do this by itself as it is an expensive, so Now the
    // programmer knows where he's using, and by using this your program might get slow.
    // Now the table is cloned in _s2
    println!("s1 is {s1}");

    // COPY TRAIT -->
    //
    //

    let num = 10;
    let result = add(num);

    let name = String::from("droovvv");

    let s: String = gives_ownership();

    takes_ownership(name);

    let s2 = takes_and_gives_back(s);

    println!("s = {s2}");

    println!("The num is {num} and result is {result}");
    // println!("Value of name is {name}");
    // This will give the same error because the ownsership of var name has changed and moveed
    // to that fn
    // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
    // println!("s = {s}");
    //

    let str = String::from("Naughty America");
    let (str, len) = calculate_len(str);
    println!("The len of {str} is {len}");
    // This fn creates an internal variable and gives its ownership to us
}
fn gives_ownership() -> String {
    let s = String::from("This is a string from takes ownership fn");
    s // we r returning "s"
}

fn takes_ownership(s: String) {
    println!("Inside ownership of {s}");
}

fn takes_and_gives_back(s: String) -> String {
    println!("S in takes_and_gives_back = {s}");
    s
}

fn add(x: i32) -> i32 {
    x + 10
} // Here, some_integer goes out of scope. Nothing special happens.

// In this we have to return a tuple because we r moving the ownership of s if we r not returning.
fn calculate_len(s: String) -> (String, usize) {
    let res = s.len();
    (s, res)
}
