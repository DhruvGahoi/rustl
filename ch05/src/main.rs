struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        active: true,
        email: String::from("random@abc.com"),
        username: String::from("random"),
        sign_in_count: 1,
    };

    // If you want to update smth then u have to make the whole object mut, for ex
    // let mut user1{...}
    // user1.email = String::from("anotherrandom@gmail.com")

    let rect = Rectangle {
        width: 32,
        height: 50,
    };

    let area = calc_area(&rect);

    println!("The area is {}", area);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Can also use shorthand like
        username,
        email,
        // username: username,
        // email: email,
        sign_in_count: 1,
    }
}

fn calc_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
