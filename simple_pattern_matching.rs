fn main() {
    let number = 5u32;

    let size = match number {
        0 => "none",
        2 | 3 => "tiny",
        4...7 => "small",
        8...20 => "medium",
        _ => "large"
    };

    println!("The size is: {}", size);
}
