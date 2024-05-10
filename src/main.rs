struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    let email = String::from("anotheremail@example.com");
    let name = String::from("Ria");
    let user2 = build_user(email, name);
    let user3=User{
        ..user2
    };
}

fn build_user(email: String, name: String) -> User {
    User {
        active: true,
        username: name,
        email: email,
        sign_in_count: 1,
    }
}
