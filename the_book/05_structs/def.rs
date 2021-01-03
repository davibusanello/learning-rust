struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let user3 =  build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}

fn build_user(email: String, username: String) -> User {
    // Struct shorthand properties
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
