use rectangles::Rectangle;

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // dbg!(&rect1);

    // println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 40,
        height: 100,
    };

    println!("Area of rect2 is: {}", rect2.area());

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    let rect3 = Rectangle::square(30);

    println!("Area of rect3 is: {}", rect3.area());

    println!("Can rect 2 hold rect3? {}", rect2.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
