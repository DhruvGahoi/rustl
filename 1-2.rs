fn main(){
    // In general, {} this is replaced with args.
    println!("{} days", 31);    
   
    // POSITIONAL ARGUMENTS                           {0}     {1}
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    // NAMED ARGUMENTS
    println!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
    );

    //FORMATTING in rust
    println!("Base 10:                      {}",    69420); // 69420
    println!("Base 2  (binary):             {:b}",  69420); // 10000111100101100
    println!("Base 8  (octal):              {:o}",  69420); //207454
    println!("Base 16 (hexadecimal):        {:x}",  69420); //10f2c
   
    // JUSTIFY TEXT(WIDTH)
    println!("{number:>20}", number=1);

    //PADDING NUMBERS WITH ANY NUMBER
    println!("{number:0>5}", number=1); // 00001
    // Left adjust with reversing the sign
    println!("{number:0<5}", number=1); // 10000

    // Named argument in SPECIFIER $ sign
    println!("{number:0>width$}", number=1, width=20);

    // Passing the correct number of arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let number: f64   = 1.0;
    let width:  usize = 5;
    println!("{number:>width$}");
}
