#![warn(dead_code)]
#![warn(unused_variables)]

struct User {
    active: bool,
    username: String,
    email:String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(String::from("randomUser123"), String::from("random@email.com"));

    let user2 = User {
        email: String::from("antoher.user.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
