fn main() {
    struct Point {
        x: i32,
        y: i32
    }

    impl Point {

        fn distance(&self) -> i32 { // called as point.distance()
            self.x + self.y
        }

        fn random() -> Point  {
            Point {
                x: 4,
                y: 4
            }
        }
    }

    let mypoint = Point::random();

    println!("distance is {}", mypoint.distance());
    //    println!("distance is {:?}", mypoint.distance());
}
