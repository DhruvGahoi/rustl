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

    // ---------------- REFERENCE AND BORROWING ------------------
    let mut strr = String::from("Hello");
    let lent = calculate_length(&mut strr);
    println!("Length of {strr} is {lent}");

    // Mutable references have one big restriction:
    let mut strrr = String::from("whwiwfds");
    let str1 = &mut strrr;
    // if you have a mutable reference to a value, you can have no other references to that value.
    // let str2 = &strrr;
    // let str3 = &strrr;

    str1.push_str(" wiuhiuwhi");
    println!("s1 {s1}");

    // Dangle
    // let ref_to_nothing = dangle();

    // ----------------- THE SLICE TYPE ---------------

    let mut s = String::from("Hello World");
    let word = first_word(&s); // value will be 5
                               // s.clear(); // empties the string
                               //
                               // Here the problem is, String is empty but the value is still 5, which is totally invalid
    println!("length of {s} is {word}");

    // A string slice is a reference to part of a String

    let strrrr = String::from("hello world");
    let hello = &strrrr[0..5];
    let world = &strrrr[6..11];

    println!("{hello} is {world}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
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

// REFERENCE AND BORROWING
// What we are doing here is, passing the reference of that object so it doesnt need to have the
// ownership of that, and instead of returning the tuple, we can simply return the len
fn calculate_length(s: &mut String) -> usize {
    //                 ^^^^ we can also give the mutable reference so that we have the perms to
    //                 mutate this String
    //But we also need to pass the &mut at the top, so we have the mut reference
    // // s is a reference to a String
    s.push_str("Hello there");
    let res = s.len();
    res
} // // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn dangle() -> &String {
// dangle returns a reference to a String

// let s = String::from("hello"); // s is a new String

// &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// ------------- THE SLICE TYPE -------------------

// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

fn first_word(s: &str) -> &str {
    // convert our String to an array of bytes
    let bytes = s.as_bytes();
    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple
    //  index, element
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
