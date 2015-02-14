let b = 3i32;
let a = if b > 1 {
    let mut c = b - 5;
    c = c * b;
    c
} else {
    li32
}

fn greater(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
