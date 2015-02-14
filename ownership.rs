fn main() {
    let x = vec![1u32, 2u32, 3u32];
    let y = x;

    println!("y is {:?}", y);
    // println!("x is {:?}", x);   

    let p = vec![1u32, 2u32, 3u32];
    let q = p.clone();

    println!("q is {:?}", q);
    println!("p is {:?}", p)
}
