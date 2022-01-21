#![allow(unused)]
use super::utils::wait;
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
pub fn structs() {
    let mut user1 = User {
        active: true,
        username: String::from("user_name"),
        email: String::from("user@name.rust"),
        sign_in_count: 1,
    };
    user1.email = String::from("other@name.rust");
    let mut user2 = build_user(String::from("my@email.org"), String::from("my_name"));
    let user3 = User {
        email: String::from("email@email.com"),
        ..user1
    };

    wait("structs: rectangles");

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area_from_values(width1, height1)
    );
    wait("structs: tuples instead?");
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_from_tuples(rect1)
    );

    wait("structs: rectangles-structs");
    let rect2 = Rectangle{
        width: 30,
        height: 50,
    };
    dbg!(&rect2);
    println!("rect2 is {:?}", rect2);

    println!("self-called area calculation: {}", rect2.area());

    let rect3 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle{
        width: 60,
        height: 45,
    };
    println!("Can rect2 hold rect3: {}", rect2.can_hold(rect3));
    println!("Can rect2 hold rect4: {}", rect2.can_hold(rect4));
    println!("Square by associated function: {:?}", Rectangle::square(9));

}

fn area_from_values(p0: i32, p1: i32) -> i32 {
    p0*p1
}
fn area_from_tuples(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}