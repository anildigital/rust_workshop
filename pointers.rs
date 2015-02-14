fn main() {
    let mut z = &3;
    {
        let y = 1;
        let x = &y;
        z = x; // Error
    }
}
