struct Point {
    x: i32,
    y: i32
}


enum Shape {
    Circle(Point, u32),
    Rectangle(Point, Point)
}


fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None

    // }
    let origin = Point {x: 0, y: 0};
    let mut circ = Shape::Circle(origin, 10);
    // circ = Shape::Ractangle(origin, origin);
}
