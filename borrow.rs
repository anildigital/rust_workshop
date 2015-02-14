fn main() {
    // let x = vec![1u32, 2u32, 3u32];
    // let y = x;

    // println!("y is {:?}", y);
    // println!("x is {:?}", x);


    let x = vec![1u32, 2u32, 3u32];
    let y = &x;
    let c = x.clone();

    println!("x is {:?}", x);
    println!("y is {:?}", *y);

    fn abc(x: &Vec<u32>) {
        println!("{:?}", *x);
    };

    let myvec = vec![1, 2, 3];
    abc(&myvec);

    // fn abc(x: &Vec<u) {

    // }
    

}
