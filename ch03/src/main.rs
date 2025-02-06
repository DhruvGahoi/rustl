use std::io;

fn main() {
    // --------------- 3.1 ------------------

    let mut x = 5;
    println!("The value of x is {x}.");
    x = 6;
    println!("The value of x is {x}.");

    // - Naming convention for const will be all UPPER_CASE
    // - Always you have to give type to the constant
    // - Can be declared globally w/o any scope
    // - Cannot be computed at runtime
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // SHADOWING
    // Diff bw shadowing and mut -->
    // - In shadowing we'll get a compile time error, if we accidentally try to reassign to this
    // - We can change the type of variable when shadowing but on mut we can't.
    // - variable without using the let keyword.

    let y = 5;
    let y = y + 1;
    {
        let y = y + 2;
        println!("value of y inner scope: {y}");
    }
    println!("value of y: {y}");

    let space = "    ";
    let space = space.len();
    println!("The space is: {space}");

    // --------------- 3.2 ----------------
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time
    let guess: u32 = "42".parse().expect("Not a number!");

    // -----> Scalar type: Single Value (int, float, bool, char)
    // 1. Integers : i8, i16, i32, i64, i128, isize for signed and  add u for unsigned.

    // Note : In case when the int is overflowed like (255, 256) then in normal mode the
    // compiler just panicks but in the --release mode, performs 2's compliment wrapping.

    // 2. Floating Point

    let f = 2.0; //f64 --> default
    let fl: f32 = 3.0; //f32

    // 3. Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    // 4. Boolean
    let t = true;
    let f: bool = false; // with explicit type notation

    // 5. Character
    let c = 'z';
    let z: char = 'Z'; // with explicit type notation

    // ------> Compound Type : Group multiple values

    // 1. Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // We can also access the element by indexing(period)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 2. Array
    let a = [1, 2, 3, 4, 5];
    let ar: [i32; 5] = [1, 2, 3, 4, 5];
    // same value
    let arr: [i32; 5] = [3; 5];
    // Accessing elements in array
    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of element at {index} is {element}");

    // ---------------3.3----------------------
    another_function();
    // Passing the parameters in func
    another_func(5);
    // Passing multiple parameters
    labeled_measure(5, 'h');

    // -----------Statement and Exp------------
    let r = 6;
    // Statement do not return a value.
    // We cant do this
    // let x = (let y = 6);

    let u = {
        let o = 3;
        //  Expressions do not include ending semicolons.
        //  If you add a  semicolon then Expression --> Statement
        o + 1
    };

    println!("The value of u is: {u}");

    // FUNCTION WITH RETURN VALUES
    let t = five();
    println!("The vale of t is {t}");
    let t1 = plus_one(5);
    println!("The value of t1 is {t1}");

    // -------------3.5---------------
    let number = 3;
    if number < 5 {
        println!("condn was true");
    } else {
        println!("condn was false");
    }

    let condn = true;
    // This condn will give error coz return type must be SINGLE type
    // let number = if condn { 5 } else { "six" };
    println!("The value of number is: {number}");

    // LOOPING
    // loop {
    //      println!("looooooooppppp...")
    // }
    // continue and break are same.
    //
    let mut cnt = 0;
    let res = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("The result is {res}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut rem = 10;
        loop {
            println!("rem = {rem}");
            if rem == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            rem -= 1;
        }
        count += 1;
    }

    println!("end count = {cnt}");

    let mut numss = 3;
    while numss != 0 {
        println!("{numss}");
        numss -= 1;
    }

    println!("LIFTOFF!");

    let arrrr = [10, 20, 30, 40, 50];
    for elem in arrrr {
        println!("{elem}");
    }

    // Reverse the range
    for a in (1..4).rev() {
        println!("{a}");
    }
    println!("LIFTOFF!!");
}

fn another_function() {
    println!("Another fn.")
}

fn another_func(x: i32) {
    println!("The value of x is: {x}");
}

fn labeled_measure(x: i32, y: char) {
    println!("{x}{y}");
}

// Function with return values
// Dont name the return val, but MUST DECLARE their type after "->"
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // If we put a semicolon here, it will produce an error
    // Coz we r changing it from Expression to Statement
}
