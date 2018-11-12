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

pub fn retry_result<T, G, C, E>(mut gen: G, check: C) -> Result<T, E>
    where
        G: FnMut() -> Result<T, E>,
        C: Fn(&T) -> bool,
{
    loop {
        let value = gen()?;
        if check(&value) {
            return Ok(value);
        }
    }
}
