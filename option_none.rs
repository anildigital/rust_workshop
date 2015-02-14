fn main() {
    enum Option<T> {
        Some(T),
        None
    }

    fn maybe_sqrt(x: i32) -> Option<u32> {
        if x >= 0 {
            Some(sqrt(x) as u32)
        } else {
            None
        }

    }
}
