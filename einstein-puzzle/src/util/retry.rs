pub fn retry<T, G, C>(mut gen: G, check: C) -> T
where
    G: FnMut() -> T,
    C: Fn(&T) -> bool,
{
    loop {
        let value = gen();
        if check(&value) {
            return value;
        }
    }
}
