fn main() {

    let mut x = Box::new(1u32); // On the heap
    *x = 2;

    // Type Box<u32>
    // Also gets moved, not copied

    fn abc() {
        let x = box 1; // malloc() happened
        // do stuff with x or *x
        // free() happened
    }

    fn def() -> Box<u32>{
        let x = Box::new(1); // malloc() happened
        // do stuff with x or *x
        x // x returned to outer owner  
    }

}
