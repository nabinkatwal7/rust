mod example_struct;

struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
struct AlwaysEqual;

fn build_user(email:String, username:String)->User{
    User{
        active:true,
        username,
        email,
        sign_in_count:1
    }
}

fn main() {
    let mut user1 = User{
        email:String::from("b5hF0@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1
    };
    let user2 = User{
        email:String::from("b5hF0@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let subject = AlwaysEqual;
}
