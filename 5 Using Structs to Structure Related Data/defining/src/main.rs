struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    {
        let mut user1 = build_user(
            String::from("someone@example.com"),
            String::from("someusername123")
        );

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            email: String::from("anotheremail@example.com"),
            ..user1
        };

        fn build_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 1
            }
        }
    }

    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        let subject = AlwaysEqual;
    }
}
