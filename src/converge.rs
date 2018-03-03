use error::*;

pub fn converge<T: Clone + PartialEq, F: Fn(T) -> T>(initial: T, step: F) -> T {
    let previous = initial;
    loop {
        let next = step(previous.clone());
        if next == previous {
            return next;
        }
    }
}

pub fn converge_result<T: Clone + PartialEq, F: Fn(T) -> Result<T>>(initial: T, step: F) -> Result<T> {
    let previous = initial;
    loop {
        let next = step(previous.clone())?;
        if next == previous {
            return Ok(next);
        }
    }
}
