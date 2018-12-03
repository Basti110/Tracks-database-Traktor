#[macro_export]
macro_rules! value {
    ($e:expr) => ((*$e).borrow_mut());
}