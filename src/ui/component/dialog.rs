#[derive(Clone)]
pub enum DialogResult<T> {
    Ok(T),
    Cancel,
}
