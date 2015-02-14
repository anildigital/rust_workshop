fn main(){
    let mut x = vec![1u32,2u32,3u32];
    {
        let y = &x; // the pointer was "borrowed"
        // x.push(1); // Not allowed, x is currently borrowed and cannot be mutated
        // y.push(1); // Not allowed, y is not a mutable reference
    }
    x.push(1); // The borrow was "returned", we can mutate now

    let mut x = vec![1u32,2u32,3u32];
    {
        let y = &mut x; // the pointer was "borrowed", mutably
        // x.push(1); // Still not allowed, x is currently borrowed and cannot be mutated
        // println!("x is {}", x) // also not allowed, y is mutating this
        y.push(1); // Allowed, y is a mutable reference
    }
    x.push(1) // The borrow was "returned", we can mutate now
}
