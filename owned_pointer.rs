fn main() {
    let x = Box::new(10i);
    let y = x.clone();

    println!("{}", *x);
}
