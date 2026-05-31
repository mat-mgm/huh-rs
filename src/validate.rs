/// A validation function: takes a reference to the value and returns
/// `Ok(())` on success or `Err(message)` on failure.
pub type ValidateFunc<T> = Box<dyn Fn(&T) -> Result<(), String> + Send>;

/// Always-passing validator — the default for every field.
pub fn no_op<T>() -> ValidateFunc<T> {
    Box::new(|_| Ok(()))
}
