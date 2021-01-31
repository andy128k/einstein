pub fn converge<T: Clone + PartialEq, F: Fn(T) -> T>(initial: T, step: F) -> T {
    let mut previous = initial;
    loop {
        let next = step(previous.clone());
        if next == previous {
            return next;
        } else {
            previous = next;
        }
    }
}

pub fn converge_result<T: Clone + PartialEq, F: Fn(T) -> Result<T, E>, E>(
    initial: T,
    step: F,
) -> Result<T, E> {
    let mut previous = initial;
    loop {
        let next = step(previous.clone())?;
        if next == previous {
            return Ok(next);
        } else {
            previous = next;
        }
    }
}
