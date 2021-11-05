/**
 * defining a struct
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_counter: u64,
}

fn main() {
    /**
     * Instantiating a struct. For this case, we are instantiating User
     */
    let user1 = User {
        active: true,
        username: String::from("johndoe"),
        email: String::from("someone@email.com"),
        sign_in_counter: 1,
    };

    println!("{}", user1.username);
}
