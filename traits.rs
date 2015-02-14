struct Point {
    x: i32,
    y: i32
}

trait Pointy {
    fn poke(&self, at: &str);
}

impl Pointy for Point {

    fn poke(&self, at: &str) {
        println!("Poked {}", at)
    }
}

fn poke_forever<T: Pointy>(pointy: T, at: &str) {
    loop {
        pointy.poke(at);
    }
}

fn main() {
    let p = Point {x: 2, y: 2 };
    poke_forever(p, "blah");
}
