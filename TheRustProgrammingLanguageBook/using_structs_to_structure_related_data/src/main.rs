fn main() {
    // Defining and instianting a structs

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_account: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_account: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // --snip--
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@example.com"),
        sign_in_account: user1.sign_in_account,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_account: 1,
    }
}
